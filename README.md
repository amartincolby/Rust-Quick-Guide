# Rust Quick Guide

This will be a quick guide to learning Rust. I'm beginning work at the date of
this commit (10/23/23) and hope to have it complete by the end of 2023.

I think that the best docs are short, sweet, specific, and do not elide the
fundamentals undergirding the syntax. I tried to achieve this educational
perfection with my [ReasonML Quick Guide](https://github.com/amartincolby/ReasonML-Quick-Guide),
but ReasonML imploded with a community schism and thus caused me to abandon the
language. I still have a fondness for ReasonML because it, and its connection to
OCaml, were my introduction to the beautiful world of highly symbolic, academic
languages, and I hope to communicate that to readers of this guide.

The Rust docs are very good but somewhat verbose. I like documentation that is
simple enough to be a one-pager. I aim for [the famous Smalltalk postcard](https://richardeng.medium.com/syntax-on-a-post-card-cb6d85fabf88).
I did a previous quick guide for ReasonML, a syntax of OCaml, which profoundly
inspired Rust's semantics.

# Further Information
- [The Official Learn Rust Online Book]([https://reasonml.org/](https://doc.rust-lang.org/book/title-page.html)) *The source of truth for learning Rust.*
- [The Learn Rust Interactive Book](https://rust-book.cs.brown.edu/)

# The Guide

> “There are only two kinds of languages: bad programming languages and the
> programming languages that nobody uses.”
> 
> ― Bjarne Stroustrup (paraphrased)

I believe that we stand on the precipice of a great shift in software
engineering. The last shift was the rise of object-oriented languages over forty
years ago. Emerging as it did from the ashes of structured programming and the
spaghetti code of _The Mythical Man Month_, OOP, as it is known, was the de
rigeur paradigm just as computing exploded into the general consciousness. 
Througout the 1980s and 1990s, the massive growth of the industry suppressed
change, even as people on the proverbial edge started to realize that OOP had
fundamental problems that were not easily soluble. These problems produced one
of, if not the, greatest technical manuals in computing history: the legendary
book _Design Patterns_ by the now-famous "Gang Of Four," a title they
unfortunately share with Communist extremists from 1970s China, which you will
learn if you try to Google that phrase.

A critical analysis of _Design Patterns_ is outside the scope of this guide,
suffice it to say that many analyses have shown the problems they solve are
_linguistic_ problems and not implementation problems. To wit, the patterns
described are band-aids over a broken paradigm. _Design Patterns_ was published
in 1994, but work on it started in 1990. We are over thirty years into an
industry-wide quest to put lipstick on a pig.

But beautiful change is in the air. Now is the autumn of the old ways. To be
clear, I do not speak of AI. Indeed, AI, or at least the AI that is generating
excitement at the current moment, is firmly rooted in the old ways: the ways of
feature delivery at all costs, quality and symbolic integrity be damned. The new
ways recognize that the fundamental tools at use must be built with the goal of
symbolic integrity in mind. This means languages.

Basically, it is time for the joke about good and bad programming languages to
cease being true.

JavaScript is perhaps the ultimate manifestation of this, with Node and JS
infecting everything, everywhere.

The below text is valid Rust code. It is a copy of the code in the src directory.

This was written in VSCode and using an interactive IDE with code linting and
highlighting is recommended.

``` rust
/* Comment blocks start with slash-star,
   and end with star-slash. */
// Line comments begin with a double slash.

/* Rust was inspired by the ML family of languages. As such, variables are not, by default, variable.
```
