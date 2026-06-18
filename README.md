# chords

An effort to learn more about music theory, and design an interface that can explore music theory in unconventional ways.

## note_lib

The `note_lib` library is a rough attempt to encode music theory in various data structures. Operations such as chord inversions or scale modes are implemented.

## note_tools

The `note_tools` program is a small utility for calculating manipulations to notes or chords.

This tool allows you to add or subtract simple or compound intervals to notes. In it's initial form, you have two commands

- `note-tools add-interval <NOTE> <INTERVAL>`
- `note-tools sub-interval <NOTE> <INTERVAL>`

See `note-tools --help` for the latest info.

## Future Considerations

In the future, a chord map interface could allow users to create a mind-map of chords, and branch from them in intuitive ways such as selecting pivot notes or finding pleasing chords within a chord progression.
