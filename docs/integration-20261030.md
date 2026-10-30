# PdfCraft integration candidate, 2026-10-30

Base: 1d1cbc4ec92012767f5b48c92b451e15eb401c42

Validation: pending combined GitHub CI. No fresh manual GUI pass has been performed.
All included source heads are preserved as merge parents, retaining original authorship. No upstream PR was merged. No conflicts were manually resolved.

## Included
- [#4304](https://github.com/storytold/pdfcraft/pull/4304): M11: PDF Optimizer stores black-and-white images as CCITT Group 4 (optimize.mono-compression) (1b7164cab1fc187c5385c30f94449542fe284280)
- [#4335](https://github.com/storytold/pdfcraft/pull/4335): M11: black-and-white pictures stay one bit: Create PDF, the optimizer, and CCITT image export (210ba20fe69e701286bbfdbd924cfc01d9aaa9f4)
- [#4384](https://github.com/storytold/pdfcraft/pull/4384): M11: Monochrome Images ▸ Downsample: black-and-white images above 450 ppi are averaged down to 300, Group 4 scans included (1ad951e25bac86f77e75e060dcce34cc32a0f9a3)
- [#4404](https://github.com/storytold/pdfcraft/pull/4404): M11: the optimizer stores Group 3 fax as Group 4, and packs black-and-white RGB and two-entry indexed images to one bit (2d0a7b22411d4b61f130f6175cec918153858593)
- [#4179](https://github.com/storytold/pdfcraft/pull/4179): fix(ui): Ctrl+Z in an open paragraph undoes the document, not stale text (296b9300988fe3ef8d5bcdcb439705e80d6d7321)
- [#5300](https://github.com/storytold/pdfcraft/pull/5300): fix(ui): a press on the page belongs to what its one hit test finds, until the button is up (11bc8c56528fc3a820669d41c0eec29310cdad24)
- [#4719](https://github.com/storytold/pdfcraft/pull/4719): fix(comments): delete or edit one reply from its own menu, keeping the thread (adeb9c1cf2336c10639d8172325e708edf515856)
- [#4541](https://github.com/storytold/pdfcraft/pull/4541): M5: a moved comment is drawn where it goes while it is dragged (f0cd96e90bf696099b02201ac68f9ccb8a00c7d2)
- [#2815](https://github.com/storytold/pdfcraft/pull/2815): fix(edit): show backgrounds on pages that paint their own white backdrop (9aaac56417a2260338c7e7edb41615f1bb42b45e)
- [#4620](https://github.com/storytold/pdfcraft/pull/4620): feat(edit): Esc in an open paragraph selects its box, so Delete removes it (64fba7d177032170939861dc7eb51d8486a43264)
- [#3789](https://github.com/storytold/pdfcraft/pull/3789): Alt+Left / Alt+Right go back and forward through the view history (#3632) (08f3991ab9808b71fa015f22d49dbc4720044c78)
- [#3779](https://github.com/storytold/pdfcraft/pull/3779): Window title: mark unsaved changes, and set it on the web (#3640) (3850c9f52e073e51f8e4a1665b3ae36fae6f8e66)
- [#2960](https://github.com/storytold/pdfcraft/pull/2960): Keep lines with footnote references in paragraph reading order (#2929) (2a7e5bda268b3862b9b2329df16191473966c90e)
- [#5208](https://github.com/storytold/pdfcraft/pull/5208): M10: export a PDF to Markdown (#1595) (20d396d180a33139908a3a1fad6178ab77b90eff)
- [#4596](https://github.com/storytold/pdfcraft/pull/4596): fix(organize,print): join /Contents array pieces with a space (#3502) (d58d121170f2d650196263492c6c54a89aad39b3)
- [#3636](https://github.com/storytold/pdfcraft/pull/3636): Content streams: step over a malformed token instead of ending the page (43337c7fd5f7b5123efcebcc84de35c394726e7a)
- [#4494](https://github.com/storytold/pdfcraft/pull/4494): M2: page_render and render_preview report the render warnings next to the PNG (#4340) (5ada1249e9cdb98a438e26f8fe53c82aba9b8cc1)
- [#4013](https://github.com/storytold/pdfcraft/pull/4013): fix(engine): generate Protect keys and salts with getrandom (#4000) (c580b550555837ad6f2c1aa4152bdb5cb96cf337)
- [#5134](https://github.com/storytold/pdfcraft/pull/5134): fix(ui): keep the All tools rows clear of the scrollbar (#5133) (0c5119791db7de268258b9d25f1286b82b563f8a)
- [#2133](https://github.com/storytold/pdfcraft/pull/2133): Fix Pages sidebar select-all keyboard shortcut (#2052) (7227eb68ab383e47413f1a980f340f5132cf4144)
- [#2635](https://github.com/storytold/pdfcraft/pull/2635): Allow typed font sizes and Fill & Sign text colours (4b319bb3835709626c663b8d41b72d2f9574acc0)

## Excluded
- [#4320](https://github.com/storytold/pdfcraft/pull/4320): conflict.
- [#4805](https://github.com/storytold/pdfcraft/pull/4805): conflict.
- [#4325](https://github.com/storytold/pdfcraft/pull/4325): conflict.
- [#4180](https://github.com/storytold/pdfcraft/pull/4180): conflict.
- [#1952](https://github.com/storytold/pdfcraft/pull/1952): conflict.
- [#2529](https://github.com/storytold/pdfcraft/pull/2529): conflict with main.

Additional previously reviewed parity fixes: #2133 and #2635. #1952 was considered but conflicts.
Excluded held PRs #4648, #2877 and #3762 were not included.
Source reviews and individual hosted CI do not establish combined-build safety; the integration gate must pass.
