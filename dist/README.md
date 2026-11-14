# Pocket Notes

**A local-first notes app. No cloud, no account, no subscription, no license check. Your notes live on your device, in an open format, and they cannot be taken away.**

If you just found this file and you are wondering what it is: it is a notes app. Open `pocket-notes.html` in a browser and it works. That is the whole install.

## What it is

Pocket Notes is a standalone notes app for Mac, PC, and the web (including iPhone, iPad, and Android via Add to Home Screen). It is one self-contained HTML file — no server, no database to install, no account to create. You open it, you write, and your notes are saved on your device.

It was built to be better than the note apps it competes with (OneNote, Evernote, Bear, Logseq, Notion, Obsidian) in the ways that matter most: your data is yours, it works offline forever, it cannot go down worldwide, and it can move your notes in and out of the other apps' formats.

## How to use it

**On the web (phone, tablet, or desktop):** open `pwa/index.html` in a browser, or install it as a web app — in Safari or Chrome, tap Share → Add to Home Screen. It then runs full-screen like an app. No App Store, no developer account, no fee.

**On Mac or PC:** download the desktop app (a `.dmg` for Mac, an installer for Windows) from a GitHub Release. It is the same app, in a native shell.

**From the file itself:** open `pocket-notes.html` in any modern browser. It works offline, straight from the file.

There is no sign-in. There is no sync. There is nothing to set up.

## What you can do with it

- **Notebooks → sections → pages.** Organise notes the way you think: a notebook holds sections, a section holds pages.
- **Rich editing.** Bold, italic, underline, headings, lists, checklists, highlights. 
- **Links.** Write `[[Page Title]]` to link to another page. The app shows backlinks and unlinked mentions, so you can see what connects to what.
- **Views.** Graph (a map of your linked pages), Board (drag cards between columns), Calendar (pages by date), and Home.
- **Tags and search.** Tag pages, then search with operators like `tag:`, `@todo`, and `@images`.
- **Sticky notes, templates, voice notes, drawings, and photos.** Capture a thought quickly, start from a template, record a voice note with a transcript, draw, or insert a photo (downscaled and stored in the note).
- **Focus Mode.** One page, nothing else, until you press Escape.
- **Import and export.** Bring your notes in from other apps and take them out again — see below.

## File formats — what it reads and writes

**Its own format is `.weave`.** A `.weave` file is plain text (Markdown plus YAML frontmatter) that holds a whole notebook: pages, tags, pins, clips, links, and a history of changes. One file, readable in any text editor, forever. The full spec is in `WEAVE.md`. An honest comparison of `.weave` against the formats it competes with is in `WEAVE-CONVENTIONS.md` — including the places where `.weave` is not yet superior.

**Import** (bring your notes in):
- `.weave` (native, everything preserved), `.md`, `.txt`, `.html`
- Notion: a `.zip` export (`.md` pages, subfolders become sections, images embedded, databases as `.csv`)
- OneNote: `.docx` and `.mht` exports
- Evernote: `.enex` (v19)
- Bear and Logseq: Markdown, including their conventions (v19)
- PDF: import a PDF as a page (v19)

**Export** (take your notes out):
- `.weave` (native, everything preserved), `.md`, `.txt`, `.html`
- `.docx` and `.pdf` (v19)
- `.enex` (v19, Evernote's format)

**What it cannot do:** OneNote's `.one` and `.onepkg` files are proprietary Microsoft binary and no non-Microsoft app can fully read them. The way in from OneNote is to export from OneNote as `.docx`, `.pdf`, or `.mht` first. The way out to OneNote is `.docx`/`.pdf` as a printout (visual, not editable) or `.enex` plus a community tool on Windows. That asymmetry is stated honestly, not hidden.

## Where your data lives

On your device, and nowhere else.

- **Primary storage is IndexedDB** (your browser's built-in database), with a fallback to localStorage if IndexedDB is unavailable. There is no server and nothing is uploaded.
- **Back up with Export.** Use the Storage dialog (hard-drive icon) or Export to save a `.weave` backup. Keep a copy somewhere safe — especially on iPhone/iPad, where the system can purge a home-screen web app's data if it goes unused for a long time.
- **Clear all storage** is in the same dialog. It deletes everything on the device after a confirmation. Export a backup first if you want to keep your notes.

## What it can do with your stuff — exactly

This is the part that matters most: precisely what the app, and an AI assistant working with it, can and cannot do with your notes. No vagueness.

**The app can:**
- Create, read, edit, and delete your notebooks, sections, and pages — on your device, when you ask it to.
- Save your notes to IndexedDB on your device, and read them back.
- Export your notes as `.weave`, `.md`, `.txt`, `.html`, `.docx`, `.pdf`, `.enex` — when you ask it to.
- Import notes from `.weave`, `.md`, `.txt`, `.html`, a Notion `.zip`, OneNote `.docx`/`.mht`, Evernote `.enex`, Bear and Logseq Markdown, and PDF — when you give it a file.
- Search your notes, link pages, show backlinks, and build a graph — all on your device, from the data already there.
- Delete everything, but only when you confirm it in the Clear Storage dialog (after being told to export a backup first).

**The app cannot, and never will:**
- Upload, share, or send your notes anywhere. It makes no network request that carries your notes. (It makes no network requests for your data at all.)
- Read notes you have not opened in it. It only sees what is stored in its own IndexedDB on your device.
- Change your notes without you doing it. It does not edit, reorganise, or delete anything on its own.
- Lock you in. Every export format is open, and `.weave` is plain text you can read in any editor.

**An AI assistant (like Glimmer) can, when you give it your `.weave` file:**
- Read every page, tag, pin, clip, link, and history entry in the notebook.
- Write new pages, rename pages, move pages between sections, add tags, create `[[links]]`, add tasks and clips — by producing a new `.weave` file that follows `WEAVE.md`, which you then import.
- Append to the History (never rewrite or delete it).
- Add metadata (unknown keys are preserved by the app).
- Export the notebook to another format for you, or convert a file from another app into `.weave`.

**An AI assistant cannot:**
- See your notes unless you give it the file. It has no access to the app's IndexedDB on your device. It only sees what you hand it.
- Send your notes anywhere. It works with the file you gave it, on the device you gave it on.
- Change the app itself. It can produce a `.weave` file; it cannot modify the app's code, and it cannot change your live notes except through a file you choose to import.
- Delete your notes. It can only produce a file. You decide whether to import it.

## For an AI assistant (or a developer) who found this

If you are an AI assistant helping someone who has this app, or a developer who wants to understand it, here is the short version:

- **The app is one file:** `pocket-notes.html`. It is the single source of truth. The web app (`pwa/index.html`) and the desktop app (`desktop/dist/index.html`) are byte-identical copies of it.
- **The data format is `.weave`:** plain text, documented in `WEAVE.md`. An AI can read a `.weave` file directly and write one by following that spec. The exact contract for an AI to read and write it is in `WEAVE-AI.md`. There is no API, no account, and no cloud — the AI works with the user's files, on the user's device, with the user's permission.
- **The full specification is `SPEC.md`.** The engineering brief is `BRIEF.md`. Performance targets are in `PERF-BUDGET.md`; the desktop size budget is in `desktop/SIZE-BUDGET.md`. The engineering memorandum that compares this app to the original OneNote and to a modern competitor is `ENGINEERING-MEMORANDUM.md`.
- **Version history is in `versions/`.** Each released version is forked there before the live file is overwritten (a rule since v13.1), so no version is ever lost.
- **The current version is v18** (173,526 bytes). v19 (full import/export compatibility plus AI-written `.weave` verification) is in progress.

## What it is not

- It is not a cloud service. There is no sync, no sharing, and no collaboration. That is deliberate: a local-first app cannot go down worldwide.
- It is not a subscription. There is no payment, no license check, and no feature locked behind an account.
- It is not a replacement for a word processor, a spreadsheet, or a slide deck. Those are separate apps in the same suite (Docs, Sheets, Slides, Mail), each with its own file, sharing the same `.weave` format.
- It does not do AI inside the app. Any AI works with the app's data through the open file format, outside the app, with the user's permission.

## The suite

Pocket Notes is the first app of a suite: Notes, then Docs, Sheets, Slides, and Mail last. They share one format (`.weave`, with a `kind:` field saying which kind of document a file is) so a page from Notes can open in Docs and a table from Sheets can embed in Notes. The suite overview is in `SUITE.md` (in the parent folder).

## License and ownership

Your notes are yours. The app is local-first and open-format by design so that no company, outage, or policy change can take your notes away from you.
