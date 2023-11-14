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

Programming is the arrangement of symbols. Symbols only have meaning insofar as
they are related to other symbols. Devoid of context, a symbol has no meaning
and, arguably, does not exist. Human knowledge is likewise a web of related
symbols, and to try to learn about one symbol separate from others is destined
to fail. So it is with programming, if not more so. The art of symbolic
arrangement cannot be taught without the context and history of the art. So many
tutorials and classes fail to consistently and repeatedly bring into lessons the
broader historical context for concepts, and this results in people either
failing to learn or only learning the end result in a rote, robotic way. The
high art of painting cannot be taught by just educating people in brush strokes.
The art of programming cannot be taught by just educating people in writing a
React component or a Java class.

These two goals, concise simplicity and broad context, are obviously at odds
with one another. I hope that I find a middle ground that is approachable and
easy to understand.

The Rust docs are very good but somewhat verbose. I like documentation that is
simple enough to be a one-pager. I aim for
[the famous Smalltalk postcard](https://richardeng.medium.com/syntax-on-a-post-card-cb6d85fabf88).
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
industry-wide quest to put lipstick on a pig.[1]

But beautiful change is in the air. Now is the autumn of the old ways. To be
clear, I do not speak of AI. Indeed, AI, or at least the AI that is generating
excitement at the current moment, is firmly rooted in the old ways. It is not
doing things _differently_, it is merely doing the same things more quickly. The
new ways recognize that the fundamental tools at use must be built with the goal
of symbolic integrity in mind. The foundations themselves must change. This
means languages.

Basically, it is time for the joke about good and bad programming languages to
cease being true.

Programming languages exist on a spectrum. On one end is the realm of pure
symbols. These are like mathematical formulae. They are relationships to be
computed and have no need to specify the machine on which they run. This sort of
language is often called "academic" because they are usually the focus of
esoteric computer science classes. On the other end of the spectrum is the
direct representation of machine state. This is often called assembly language,
where each statement represents a command going to the computer.

Rust starts on the academic end of the spectrum of languages and moves toward
the machine in search of maximum, real-world performance. Meanwhile Rust's
philsophical counterpart, C, starts from the machine and moves toward symbolic
purity. Indeed, the entire purpose of C was to be "portable assembly": the
thinnest possible symbolic layer over machine instruction to faciliate writing
programs for the multitude of CPU architectures that existed in the late 1970s.
There was basically an architecture for each company making commputers. The idea
of something like the x86 architecture dominating the CPU industry was mostly
unimagineable. There were other higher-level languages available at the time,
notably COBOL, Lisp, Fortran, PL/I, Algol, and Pascal, but none of them were
simple enough to be easily usable for truly general tasks, and their performance
was universaly much slower than hand-written assembly, as all highly symbolic
languages are. C was specifically created to move only as far away from the
machine as was necessary to fulfill the goal of code portability.

Unfortunately, unlike symbols, which are pure and Platonic, the machine state is
unpredictable and messy. One command can easily be incompatible with another.

I see the juxtaposition of C and Rust to be a microcosm, a synecdoche of sorts,
for the whole of computer history. The famous book _Hackers: Heroes of The
Computer Revolution_ captures it well. Even in the earliest days of computing,
philosophers and academics were aware of how to robustly build symbolic
machines. The problem was that representing these robust symbolic structures
required a great deal of processing power that, in the 1950s and 1960s, was in
short supply. As such, the excitement and successes of the real world was
defined by the first "hackers," who were people less concerned with what symbols
could do and more concerned with the cool things that could be achieved with
direct control over the machine. This divide between the symbolic purists and
the hackers would continue into every successive generation of geeks. The
divide goes back even further, though, with the very first people in the field,
Ada Lovelace and Charles Babbage, separated along the same lines. Lovelace was a
self-described metaphysician who recognized the philsophical possibilities in
Babbage's machine, while Babbage himself cared about the machine itself. The
narrative thread connecting Rust and C goes to the very beginning of this
industry.

(1) While _Design Patterns_ vis-a-vis languages is open to significant critique,
the concepts discussed in the book are still valuable, especially as regards
distributed systems. Solving problems of multi-service systems necessarily
cannot be solved with any current language and must be solved in the
implementation. The next great language may be one that turns archiectural
concerns into linguistic concerns, yet again obviating the book, but for the
time being, it remains a must-read.

The below text is valid Rust code. It is a copy of the code in the src directory.

This was written in VSCode and using an interactive IDE with code linting and
highlighting is recommended.

``` rust
/* Comment blocks start with slash-star,
   and end with star-slash. */
// Line comments begin with a double slash.

/* Rust was inspired by the ML family of languages. As such, variables are not, by default, variable.
```
