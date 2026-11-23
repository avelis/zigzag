# Trim Before

An object's advanced Stitch/Sewing row has independent **Trim Before** and
**Trim After** controls. Start/end markers continue to choose stitch endpoints.
Turning Trim Before off restores automatic travel planning; it does not forbid
a cut that the planner needs. Edits use the existing document undo/replan flow.

SVG IDs store `trim_before=1` (force), `trim_before=0` (automatic), or no token
(inherit/default). Bare `trim_before` also means force. Like `trim` (the existing
Trim After token), it inherits from groups; an explicit zero overrides an
inherited request. MCP `set_object` accepts `trim_before: true`, `false`, or
`null` to reset; `get_design` reports an explicit boolean in each object’s
`settings` (this source has no standalone `get_object` tool).

A forced cut applies once, at the object's first sewn piece, only when there
is an earlier run in the current thread block. It overrides manual `route_mm`
travel. A preceding Trim After and this Trim Before produce one cut. Empty and
Skip objects do not add cuts or consume a preceding cut request. Joined satin
keeps a request at its entry; a request on a later column creates an explicit
object boundary. Exporters continue to consume the machine-independent
`Run.trim_before` flag; machine interpretation remains format/device dependent.

This is the first bounded sequence improvement. The sequence list still groups
color blocks. Per-object sequence rows/context menus, broader stitch start/end
planning improvements, Complex Fill, Quick Mode, and workspace flexibility are
separate work. No Tajima code or assets are included.
