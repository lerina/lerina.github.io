Screen width based styles can be applied to most classes. They are

-ns (not small > 48em) 

-m (medium >48em <64em)

-l (large >64em).


Tachyons is mobile-first and small is left out as it's the default.

examples: pa7-ns, indent-ns

## Typography
|  **base** | **css-name** | **modifiers** | **example** | **notes** |
| :---: | --- | --- | --- | --- |
|  f | font-size | 1 to 7, -5, -6 | f1, f-5 | 1 to 7 range from 6rem(96px) down to .875rem(14px); -5 and -6 are for headlines |
|  measure | max-width | narrow, wide | measure, measure-wide | sets characters per line to 45, 66, 80 |
|  indent | text-indent |  |  | indent's paragraph |
|  truncate | overflow |  |  | truncates to measure width or container |
|  lh | line-height | solid, title, copy | lh-title | sets height to 1, 1.25, 1.5 |
|  tracked | letter-spacing | tight, mega | tracked, tracked-tight | sets spacing to .1em, -.05em, .25em; only add tracking with all caps |
|  fw, normal, bold | font-weight | 1 to 9 | fw3, normal | 1 to 9 = 100 to 900; weight should decrease as size increases |
|  fs, i | font-style | normal | fs-normal, i | normal or italic |
|  v | vertical-align | base, mid, top, bottom | v-mid, v-base | works on inline elements |
|  t | text-align | l, r, c, j | tl, tr | left, right, center, justify |
|  tt | text-transform | c, l, u, n | ttc, ttl | capitalize, lowercase, uppercase, none |
|  strike, underline, no-underline | text-decoration |  |  |  |
|  ws-normal, nowrap, pre | white-space |  |  | nowrap clips; pre preserves whitespace |
|   | font-family |  |  | several standard system fonts are included; see docs |


## Layout
|  **base** | **css-name** | **modifiers** | **example** | **notes** |
| :---: | --- | --- | --- | --- |
|  debug |  |  |  | displays a border on element and all descendants |
|  debug-grid |  | -16, -8-solid, -16-solid | debug-grid, debug-grid-8-solid | displays a grid background for alignment |
|  border-box | box-sizing |  |  | places border on outside of element so element size is not changed |
|  p | padding | a, h, v, t, r, b, l, 0 to 7 | pa4, pv0  | all, horizontal, vertical, top, right, bottom, left; 0 to 7 doubles from .25 to 16 rem |
|  m | margin | a, h, v, t, r, b, l, 0 to 7 | mh3, mb2 | all, horizontal, vertical, top, right, bottom, left; 0 to 7 doubles from .25 to 16 rem |
|  f | float | l, r, n | fl, fr | left, right, none |
|  cf |  |  |  | clearfix - fixes float bugs |
|  d | display | n, i, b, ib, it |  | none, inline, block, inline-block, inline-table |
|  dt | display | c, row, group, column | dtc, td-row, dt-column-group | table- cell, row, row-group, column, column-group |
|  dt--fixed | table-layout |  |  | fixes table layout and sets width to 100% |
|  w | width | 1 to 5, 10 to 100, third, two-thirds | w3, w-80, w-third | 1 to 5 doubles from 1 to 16 rem; 10 to 100 is by 10s & includes 25, 33, 34, 75 |
|  fl |  |  |  | float-based grid; use with width settings; documentation incomplete |
|  mw | max-width | 1 to 9, -100, none | mw7, mw-100 | 1 to 9 doubles from 1 to 96 rem |
|  h | height | 1 to 5, -25, -50, -75, -100, auto, inherit | h3, h-50 | 1 to 5 doubles from 1 to 16 rem |
|  vh | height | -25, -50, -75, -100 | vh-25, vh-50 | view/screen height |
|  min-h | height |  |  | height 100% |
|  min-vh | height |  |  | vertical screen height 100% |
|  static | position |  |  |  |
|  relative |  |  |  |  |
|  absolute |  | --fill | absolute, absolute--fill | absolute and absolute with all sides at 0 |
|  fixed |  |  |  |  |
|  top |  | -0, -1, -2, --1, --2 | top-0, top--1 | 0, 1, 2, -1, -2 rem |
|  right |  | -0, -1, -2, --1, --3 | right-1, right--2 | 0, 1, 2, -1, -2 rem |
|  bottom |  | -0, -1, -2, --1, --4 |  | 0, 1, 2, -1, -2 rem |
|  left |  | -0, -1, -2, --1, --5 |  | 0, 1, 2, -1, -2 rem |

## Flexbox
Use a combination of flex and widths to create grids.

|  **base** | **css-name** | **modifiers** | **example** | **notes** |
| :---: | --- | --- | --- | --- |
|  flex | display |  |  |  |
|  inline-flex | display |  |  |  |
|  flex | flex-direction | column, row, column-reverse, row-reverse | flex-column, flex-row-reverse |  |
|  flex | flex-wrap | wrap, nowrap, wrap-reverse | flex-wrap, flex-nowrap |  |
|  items | align-items | start, end, center, baseline, stretch | items-start, items-end |  |
|  self | align-self | start, end, center, baseline, stretch | self-center, self-stretch |  |
|  justify | justify-content | start, end, center, between, around |  |  |
|  content | align-content | start, end, center, between, around, stretch |  |  |
|  order | order | -0 to -8, last | order-2, order-last | last=99999 |

## Theming
|  **base** | **css-name** | **modifiers** | **example** | **notes** |
| :---: | --- | --- | --- | --- |
|  black | color | -05, -10 to -90 | black-20, black-80 | 10 to 90 is by 10s |
|  white | color | -10 to -90 | white-30, white-70 | 10 to 90 is by 10s |
|  bg-black | background-color | -05, -10 to -90 | bg-black-05, bg-black-50 | 10 to 90 is by 10s |
|  bg-white | background-color | -10 to -90 | white-10, white-90 | 10 to 90 is by 10s |
|   | color | -inherit | moon-gray, hot-pink | lots of colors in the docs (they don't always match CSS colors) |
|  bg | background-color | -inherit | bg-blue, bg-inherit | lots of colors in the docs (they don't always match CSS colors) |
|  dim | :hover |  |  |  |
|  glow | :hover |  |  |  |
|  hide-child | :hover |  |  | reveals child on hover |
|  underline-hover | :hover |  |  |  |
|  grow | :hover | -large | grow, grow-large |  |
|  pointer | :hover |  |  | show pointer icon on hover |
|  shadow-hover | :hover |  |  |  |
|  bg-animate | :hover |  |  |  |
|  cover | background-size |  |  |  |
|  contain | background-size |  |  |  |
|  b | border-style | a, t, r, b, l, n | ba, bl | all, top, right, bottom, left, none; uses text color of DOM node |
|  b | border-style | --none, --dotted, --dashed, --solid | b--dotted, b--solid |  |
|  b | border-color | --transparent, inherit | b--orange, b--light-yellow | lots of colors in the docs (they don't always match CSS colors) |
|  bw | border-width | 0 to 5 | bw2, bl-0 | 0 to 5 doubles from .125 to 2 rem; bt-0, br-0, bb-0, and bl-0 are also available as resets |
|  br | border-radius | 0 to 4, -100, pill, --bottom, --top, --right, --left | br3, br-100, br-pill, br--right | 0 to 4 doubles from .125 to 1 rem; pill = radius 9999; directions set side to zero |
|  shadow | box-shadow | -1 to -5 | shadow-2, shadow-3 | numbers represent styles, not sizes |
|  o | opacity | -0, -025, -05, -10 to -100 | o-025, o-80 | 10 to 100 is by 10s |

## Elements
|  **base** | **css-name** | **modifiers** | **example** | **notes** |
| :---: | --- | --- | --- | --- |
|   |  |  |  | see docs for images |
|  link |  |  |  | removes underline and sets a special hover animation speed |
|  list | list-style-type |  |  | resets link style and removes bullets |
|  input-reset |  |  |  | resets form input style for mobile |
|  button-reset |  |  |  | resets button style for mobile |
|  collapse |  |  |  | collapses border and removes border spacing |
|  stripe |  | -light, -dark | stripe-light, stripe--moon-gray | adds stripes; see docs for colors |

Copyright (c) 2019-present by [Lee Nathan](https://onetap.software)

Released under the [MIT License](https://opensource.org/licenses/MIT)