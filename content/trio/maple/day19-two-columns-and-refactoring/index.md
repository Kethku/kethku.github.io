+++
title = "Day19 - Two Columns And Refactoring"
description = "Refactored the Room Endpoint Plus Some CSS"
date = 2023-04-28
+++

Quick post today as its very late and my day was pretty
chaotic. I worked on the site style a bit more in
preparation for the dynamic portion of the site and
refactored the endpoint I wrote yesterday for the same
reason.

## Two Columns

The new site structure has the main content to the left with
a dynamic column that is sticky on the right side. When on
mobile the dynamic column is hidden away because its not
strictly required. To achieve these three goals, I
refactored the site style to use a css grid layout and a
media query to add the extra css when the viewer is big
enough.

```sass
// If screen is small, don't show the sidebar
// just have a single column of content centered on the screen
body
    background-color: $background
    display: grid
    justify-content: center
    grid-template-columns: minmax(10px, $contentWidth)
    grid-template-rows: auto auto auto 1fr
    grid-template-areas: "header" "main" "footer" "."

#sidebar
    display: none

@media (min-width: $contentWidth)
    // If the screen is bigger than 11 inches show the sidebar
    // fit-content is used so that if the sidebar doesn't exist,
    // the main content is still centered on the screen
    body
        display: grid
        grid-template-columns: 1fr $contentWidth 10px fit-content(3in) 1fr
        grid-template-rows: auto auto auto 1fr
        grid-template-areas: ". header header header . " ". main . sidebar ." ". footer footer footer ." ". . . . ."

    #sidebar
        display: block
        align-self: start
        grid-area: sidebar
        top: 0
        position: sticky
```

The key points are using the @media query with the same body
style just updated to have the extra sidebar column. I used
fit-content(3in) so that if there isn't any text in the
sidebar, that column collapses to zero width giving the main
content more space.

`top: 0` and `position: sticky` puts the sidebar at the top
of the screen so that the dynamic content is always visible
even when the user has scrolled a bit.

For now these changes aren't really that visible, once the
sidebar gets some actual content in it, the user will be
able to see messages when other visitors arrive and may even
be able to interact with them. Initially I just want live
notifications and intractability with the expectation that
basically nobody will be on the site. But eventually I would
like to handle scaling out to many users.

## Refactoring

With the site design out of the way for now, I also worked
on refactoring the room endpoint. Yesterday's code was very
simple but not very extensible or readable. So today I
pulled the Visitor, Room, and World structs out into their
own modules. I moved the mutation functions into the world
struct by creating `get_visitor` and `move_visitor` helpers
to wrap up the long function from yesterday into higher
level pieces. The endpoint itself then becomes extremely
simple.

```rs
#[get("/room/<visitor_id>/<path..>")]
fn visit_room(visitor_id: usize, path: PathBuf, world: &State<World>) -> Json<RoomSnapshot> {
    let visitor = world.get_visitor(visitor_id);
    *visitor.last_seen.write() = Instant::now();
    let room = world.move_visitor(&visitor, &path);
    Json(room.snapshot(visitor.id))
}
```

My hope is that as more functionality is created, they can
follow this same pattern of simple operations built up of
basic building blocks.

But for now its very late and I must sleep. Till tomorrow  
Kay
