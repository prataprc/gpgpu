Notes on fonts, layout, rasterization and rendering.

* font-type, sort, compositors, impress onto paper.
* hot metal typesetting, combined casting and compositing into one automated activity.
* IBM character 9x14 rectangle of bitmap matrix.
* baseline & bounding box.
* global scripts.
* vector graphics - outlines, counters and shaping - scale to size - rasterization
* vector representation - lines, curves and geometric operations.
* control-points: start-point, line, sharp-corner, curve, tangent.
* variable-font technology.

* IKARUS - outer and inner outlines, fills in the interior of the outline with ink.
* antiquing in IKARUS.

* Donald Knuth, Metafont.
* pen, shape of pen, trace the shape based on algorithmic relationship between points.
* parametrized fonts.

* Interpress - create a language of printing shared among manufacturers.
* Postscript, type-1, type-3, it is a language.
* hinting (type-1), for small-point sizes - stems, baseline, x-height, cap-height alignment.
* cubic-bezier-curves type 1, start-point, end-point, two off-curve control points.
* quadratic-bezier-curve type 3, start-point, end-point and one off-curve control-point.
* multiple-masters in type-1 font.
* instances = variations between multiple masters.

* TrueType, Apple Advanced Typography.
* ligatures, contextual substitutions, alternate glyph, font variations.
* Uniscribe - true-type font renderer on windows.
* Opentype = Post-script (type-1) + true-type, with interoperability and compatibility.
* typeface family - upright-roman, italic, related-bold.
* typographic differentiation using speed (italic) and weight (latin fonts).
* 9 weights, 5 widths and choice of regular and oblique = 90 variations.

* small-cap variant. swash character.
* line-spacing. letter-spacing. word-spacing, space-of-different-width, hyphenation.
* word-break

```
              +---------+     +-----------------+
              | Unicode |  +--| Font Management |
              +---------+  |  +-----------------+
                    |      |
                    |      |
                    V      V
                  +---------+     +--------+
                  | Shaping |<----| Layout |
                  +---------+     +--------+
                       |
                       V
                  +------------+
                  |  Language  |
                  | Processing |
                  +------------+
                       |             +-------------+
                       |      +----->| Hyphenation |
                       |      |      +-------------+
                       V      V
                  +---------------+    +---------------+
                  | Line Breaking |<-->| Justification |
                  +---------------+    +---------------+
 +---------+              |
 | Hinting |              |
 +---------+              V
      ^             +-----------+   +---------------+
      *------------>| Rendering |<--| Rasterisation |
                    +-----------+   +---------------+
```

Handling fonts is essentially taking the font-data, unicode-standard,
shaping them, applying layout and rasterizing them.

* fontconfig for linux to deal with font-data.
* harfbuzz for shaping fonts.

* glyphs, combining glyphs, and character.

* base-line, position.
* em-square, horizontal-advance, bounding-box, side-bearings, kerning.
* x-height, cap-height, ascender, descender.

* colorfont.wtf

Bezier-curves
-------------

Ref1: https://simoncozens.github.io/fonts-and-layout//
Ref2: https://pomax.github.io/bezierinfo/
