# `ics-filter`

okay, so the purpose of this project is pretty simple: if you have a calendar you wish to subscribe
to a subset of the events from. The goal is to provide an api to which you pass an ics url and
filter options, and you get the filtered ics back.

## a note on quality

currently, the rust component is very dodgily written, and is only intended as a bare minimum "at
least it works" implementation. for example; right now it doesn't understand `SUMMARY` fields
which span multiple lines. maybe eventually i'll fix this, the code style, and maybe add more
filtering options.
