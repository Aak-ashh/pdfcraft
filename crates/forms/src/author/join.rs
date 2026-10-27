//! Renaming a radio button to an existing radio group's name makes it a button of that group
//! (#4949): radio buttons are one group when they are widgets of one field (ISO 32000-2
//! §12.7.5.2.4), and a button's export value is the name of its "on" appearance state.

use std::collections::HashSet;

use super::lifecycle::{drop_from_calculation_order, split_merged};
use super::tree::detach;
use super::*;

/// The keys a field that is also its own widget sheds when it becomes only a widget. `/AA`
/// stays: a widget's own triggers live there.
const FIELD_ONLY_KEYS: [&[u8]; 15] = [b"T", b"FT", b"Kids", b"Ff", b"V", b"DV", b"Opt", b"TU", b"TM", b"MaxLen", b"Q", b"DS", b"RV", b"TI", b"I"];

/// The most "ChoiceN" names tried for a renamed export value.
const MAX_FRESH_STATES: usize = 10_000;

/// Whether `field` holds `/Opt`, which its buttons' export values index (ISO 32000-2 Table 230).
fn has_opt(doc: &Document, field: ObjRef) -> bool {
    doc.get(field).as_dict().is_some_and(|d| d.contains(b"Opt"))
}

/// Refuse what [`join_radio_group`] can't do, before anything is written.
pub(super) fn check_join(doc: &Document, f: &Field, target: &Field) -> Result<(), FormError> {
    for x in [f, target] {
        if has_opt(doc, x.obj) {
            return invalid(format!("{} lists its export values in /Opt, so its buttons can't move between groups", x.name));
        }
        if x.widgets.is_empty() {
            return invalid(format!("{} has no button", x.name));
        }
    }
    if target.locked() {
        return invalid(format!("{} is locked: unlock it first", target.name));
    }
    Ok(())
}

/// Move every button of radio field `f` into radio field `target`. Each keeps its export value
/// unless a button of `target` already has it; then it takes the first free "ChoiceN", as a
/// button added to a group does. A moved button shows selected only if it is `target`'s
/// selected value. `f` leaves the form; its own value and field-level properties go with it,
/// and `target`'s stay. Call [`check_join`] first.
pub(super) fn join_radio_group(doc: &mut Document, f: &Field, target: &Field) -> Result<(), FormError> {
    // A group that is its own one button gets a field with a separate widget first, so it can
    // hold more than one.
    if let [only] = target.widgets.as_slice()
        && only.obj == target.obj
    {
        split_merged(doc, target, only)?;
    }
    let mut taken: HashSet<String> = target.widgets.iter().filter_map(|w| w.on_state.clone()).collect();
    let selected = target.value.first().cloned();
    detach(doc, f.obj)?;
    let moved: HashSet<ObjRef> = f.widgets.iter().map(|w| w.obj).collect();
    drop_from_calculation_order(doc, |r| r == f.obj && !moved.contains(&r))?;
    let mut kids = Vec::with_capacity(f.widgets.len());
    for w in &f.widgets {
        let mut d = doc.get(w.obj).as_dict().cloned().unwrap_or_default();
        if w.obj == f.obj {
            for k in FIELD_ONLY_KEYS {
                d.remove(k);
            }
        }
        let mut state = w.on_state.clone();
        if let Some(old) = w.on_state.as_deref().filter(|s| taken.contains(*s)) {
            let fresh = (1..=MAX_FRESH_STATES).map(|i| format!("Choice{i}")).find(|c| !taken.contains(c));
            if let Some(fresh) = fresh
                && rename_state(doc, &mut d, old, &fresh)
            {
                state = Some(fresh);
            }
        }
        if let Some(s) = &state {
            taken.insert(s.clone());
        }
        let on = state.is_some() && state == selected;
        let shown = match (&state, on) {
            (Some(s), true) => Object::Name(crate::name_bytes(s)),
            _ => Object::name("Off"),
        };
        d.set(b"AS".to_vec(), shown);
        d.set(b"Parent".to_vec(), Object::Ref(target.obj));
        doc.set(w.obj, Object::Dict(d));
        kids.push(Object::Ref(w.obj));
    }
    doc.update_dict(target.obj, |d| {
        let mut list = d.get(b"Kids").and_then(|k| k.as_array().cloned()).unwrap_or_default();
        list.extend(kids);
        d.set(b"Kids".to_vec(), Object::Array(list));
    })?;
    Ok(())
}

/// Rename the appearance state `old` to `new` in the widget dictionary `d` (`/AP` `/N`, `/D`,
/// `/R`). A shared `/AP` or state dictionary is copied into `d` rather than renamed for its other
/// users. Whether `/N` had the state: when it didn't, nothing is changed.
fn rename_state(doc: &Document, d: &mut Dict, old: &str, new: &str) -> bool {
    let (old, new) = (crate::name_bytes(old), crate::name_bytes(new));
    let Some(mut ap) = d.get(b"AP").and_then(|a| doc.resolve(a).as_dict().cloned()) else { return false };
    let states = |ap: &Dict, key: &[u8]| ap.get(key).and_then(|s| doc.resolve(s).as_dict().cloned());
    if !states(&ap, b"N").is_some_and(|n| n.contains(&old)) {
        return false;
    }
    for key in [&b"N"[..], b"D", b"R"] {
        let Some(mut sub) = states(&ap, key) else { continue };
        if let Some(v) = sub.remove(&old) {
            sub.set(new.clone(), v);
            ap.set(key.to_vec(), Object::Dict(sub));
        }
    }
    d.set(b"AP".to_vec(), Object::Dict(ap));
    true
}
