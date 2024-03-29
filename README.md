![Rust Logo](rust-social-wide.jpg)

# Rust Quick Guide

This tutorial is a work in progress.

This tutorial is not meant for people absolutely brand new to programming. It is
intended for people with at least some degree of experience and makes frequent
connections to the languages Go, C, C++, TypeScript, and JavaScript since I see
those demographics as the people most likely to come here. I will focus slightly
more on TypeScript and JavaScript since I see a move from that language to be of
the greatest value and sense a readiness in the web development community to
move to better languages.

I think that the best docs are short, sweet, and specific, elide programming
basics, but do not elide the symbolic fundamentals undergirding the syntax. The
reasons for this are twofold. First, people already accustomed to programming
will have very short attention spans when retreading territory. Second, it is
not the basics that are of great value on the open market; it is knowledge of
_the arcane_. I argue that all human endeavors have _arcana_, the secret
knowledge that separates the true practitioners from the dillettantes. As such,
even the simplest introductions to a language must include some degree of
arcane knowledge to be of any worth. I tried to achieve this educational
perfection with my [ReasonML QuickGuide](https://github.com/amartincolby/ReasonML-Quick-Guide),
but ReasonML imploded with a community schism and thus caused me to abandon the
language. I still have a fondness for ReasonML because it, and its connection
to OCaml, were my introduction to the beautiful world of highly symbolic,
academic languages, and I hope to communicate that to readers of this guide.

Programming is the arrangement of symbols. Symbols only have meaning insofar as
they are related to other symbols. Devoid of context, a symbol has no meaning
and, arguably, does not exist. Human knowledge is likewise a web of related
symbols, and to try to learn about one symbol separate from others is destined
to fail. So it is with programming, if not more so.

The art of symbolic arrangement cannot be taught without the context and history
of the art. Many tutorials and classes fail to consistently and repeatedly
bring into lessons the broader historical context for concepts, and this results
in people either failing to learn or only learning the end result in a rote,
robotic way. The high art of painting cannot be taught by just educating people
in brush strokes. The art of programming cannot be taught by just educating
people in writing a React component or a Java class.

These two goals, concise simplicity and broad context, are obviously at odds
with one another. This is especially true with a language as rich as Rust. I
hope that I find a middle ground that is approachable and easy to understand.

The Rust docs are good but verbose. I like documentation that is simple enough
to be a one-pager. I aim for [the famous Smalltalkpostcard](https://richardeng.medium.com/syntax-on-a-post-card-cb6d85fabf88).
I did a previous quick guide for ReasonML, a syntax of OCaml, which profoundly
inspired Rust's semantics.

# Further Information
- [The Official Learn Rust Online Book](https://doc.rust-lang.org/book/title-page.html) *The source of truth for learning Rust*
- [The Learn Rust Interactive Book (includes helpful quizes)](https://rust-book.cs.brown.edu/)
- [A great blog from a Rust consultancy](https://corrode.dev/blog/)
- [The Rust Async Docs](https://rust-lang.github.io/async-book/) *These have been perpetually unfinished*
- [The Tokio Docs](https://tokio.rs/tokio/tutorial) *Rust's most popular async runtime and what most people mean when they say async*
- [The official docs for Rustdoc](https://doc.rust-lang.org/rustdoc/)
- [The official docs for Cargo](https://doc.rust-lang.org/cargo/)
- [The official docs for Clippy](https://doc.rust-lang.org/clippy/)
- [The official docs for Actix-Web](https://actix.rs/docs) *Rust's most popular web framework. Relies on Tokio.*
- [Rayon Github](https://github.com/rayon-rs/rayon) *Rayon is a high-level library for making sequential processes concurrent*
- [The Little Book Of Rust Macros](https://veykril.github.io/tlborm/) *Documentation specifically for macros*

# The Guide

> “There are only two kinds of languages: bad programming languages and the
> programming languages that nobody uses.”
> 
> ― Bjarne Stroustrup (paraphrased)

I believe that we stand on the precipice of a great shift in software
engineering. The last shift was the rise of object-oriented languages over forty
years ago. Emerging as it did from the ashes of structured programming and the
spaghetti code of _The Mythical Man Month_, OOP, as it is known, was the _de
rigeur_ paradigm just as computing exploded into the general consciousness. 
Throughout the 1980s and 1990s, the massive growth of the industry suppressed
change, even as people on the proverbial edge started to realize that OOP had
fundamental problems that were not easily soluble. These problems produced one
of the greatest, if not _the_ greatest, technical manuals in computing history:
the legendary book _Design Patterns_ by the now-famous "Gang Of Four,"
a title they unfortunately share with Communist extremists from 1970s China,
which you will learn if you try to Google that phrase.

A critical analysis of _Design Patterns_ is outside the scope of this guide,
suffice it to say that many analyses have shown the problems they solve are
_linguistic_ problems and not implementation problems. To wit, the patterns
described are band-aids over a broken paradigm. Solving linguistic problems
with structure is not new. The humorous maxim of Greenspun's Tenth
Rule<sup>[[0](#fn0)]</sup> specifically addresses this and he was talking about _Lisp and
Fortran!_. _Design Patterns_ was published in 1994, but work on it started in
1990. We are over thirty years into an industry-wide effort to put lipstick on
a pig.<sup>[[1](#fn1)]</sup>

But beautiful change is in the air. Now is the autumn of the old ways. To be
clear, I do not speak of AI. Indeed, AI, or at least the AI that is generating
excitement at the current moment, is firmly rooted in the old ways. It is not
doing things _differently_, it is merely doing the same things more quickly. The
new ways recognize that the fundamental tools at use must be built with the goal
of symbolic integrity in mind. The foundations themselves must change.

Basically, it is time for the joke about good and bad programming languages to
cease being true.

--------------------------------------------------------------------------------

Programming languages exist on a spectrum. On one end is the realm of pure
symbols. These are like mathematical formulae. They are relationships to be
computed and have no need to specify the machine on which they run. This sort of
language is often called "academic" because they are usually the focus of
esoteric computer science classes. On the other end of the spectrum is the
direct representation of machine state. This is often called assembly language,
where each statement represents a direct command going to the computer. It is
fimly rooted in the physical world.

Rust starts on the academic end of the spectrum of languages and moves toward
the machine in search of maximum, real-world performance. Meanwhile, Rust's
philsophical counterpart, C, starts from the machine and moves toward a more
symbolic representation of concepts in search of easier use. Indeed, the stated
purpose of C was to be "portable assembly": the thinnest possible symbolic
layer over machine instruction to faciliate writing programs for the multitude
of CPU architectures that existed in the late 1970s. There was basically an
architecture for each company making computers. The idea of something like the
x86 architecture dominating the CPU industry was unimagineable. There were
other higher-level languages available at the time, notably COBOL, Lisp,
Fortran, PL/I, Algol, and Pascal, but none of them were simple enough to be
easily usable for truly general tasks, and their performance was universally
much slower than hand-written assembly, as all highly symbolic languages were.
C was specifically created to move only as far away from the machine as was
necessary to fulfill the goal of code portability. This raw performance is why,
even after all this time, C remains the _ne plus ultra_ for lower-level code
writing.<sup>[[2](#fn2)]</sup>

I see the juxtaposition of C and Rust to be a microcosm, a synecdoche of sorts,
for the entirety of computing history. The famous book _Hackers: Heroes of The
Computer Revolution_ captures it well. Even in the earliest days of computing,
philosophers and academics were aware of how to robustly build symbolic
machines. The problem was that representing these robust symbolic structures
required a great deal of processing power that, in the 1950s and 1960s, was in
short supply. As such, the excitement and successes of the real world were
defined by the first "hackers," who were people less concerned with what symbols
could do and more concerned with the cool things that could be achieved with
direct control over the machine. This divide between the symbolic purists and
the hackers would continue into every successive era of computation.<sup>[[3](#fn3)]</sup>

Unfortunately, the consequence of this is an entire industry that never fully
embraced either perspective. As computing power increased, higher-level
languages became more common, such as COBOL and Fortran in the 60's, BASIC and
Pascal in the 70's, Ada in the 80's, Java in the 90's, and perhaps the
Frieza-style **Final Form** of messy languages, JavaScript, in the 2000's.<sup>[[4](#fn4)]</sup> But
these languages rarely came from academic research and were instead
Frankenstein monsters that emerged from the needs of _business_ or simple
creations aimed at educating laypeople.<sup>[[5, 6, 7](#fn5)]</sup> Their _raison d'etre_
was neither geeky nor academic; it was purely human.<sup>[[8](#fn8)]</sup>

These languages represent the third major historical thread running parallel to
the struggle between the geeks and the academics: businesses. In the early days
of computing, this thread was of minor significance simply because there were
comparatively few computers and programmers in the world. But as the number of
programmers increased, so did the cultural impact of the languages being used
in the wild.

These languages powered the _usage_ of computers. That is to say that COBOL was
used by companies that used computers to write solutions for business problems.
One of the key elements, if not _the_ key element, of this thread is that
businesses were much less concerned with performance or symbolic purity; they
wanted features and they wanted them yesterday. To a great degree, C grew
dominant because it was easier to transport code between computers, pleasing
businesses.

As software exploded in importance, business concerns had an increasing effect
on languages, tools, and culture. Thus, it is no surprise that C was not the
end of linguistic progress toward easier feature delivery and exploitation.
Java, like C still among the most popular languages in the world, was
introduced in 1995 and removed even the need to compile for specific
architectures, instead opting to run all programs inside a "virtual machine."
The performance was initially abysmal, but that did not stop its ascent.
Languages like JavaScript, Python, and Ruby brought this process to its
ultimate conclusion, where code is not compiled at all, but run as-is in a
"runtime." It can be run instaneously, anywhere that supports the runtime.
Unsurprisingly, even with performance far worse than Java or C, JavaScript can
now be found running in every corner of the world.

Features über alles.

But as I said, things are changing. We spent decades focused almost entirely on
faster delivery speed. We made industry-wide decisions in service of that goal.
But just as a new generation of programmers is becoming dominant, notably having
graduated into the smoldering wreckage of the dot-com bubble and thus having a
natural skepticism for the status quo, so too are businesses realizing that
prioritizing feature delivery over other concerns can have profoundly negative
outcomes.

There are moral consequences: A bad program can produce much more CO2 than a
good program; a bad program can fail someone just when they need it most; a bad
program can literally kill people;<sup>[[9](#fn9)]</sup> a bad program can
expose users to hackers; a bad program can fail society at
large.<sup>[[10](#fn10)]</sup> There are business consequences if confidence in a
product falls. There are even organizational consequences, with burnt-out
engineers quitting. I am sure that all of those factors come into play. But I
think the real motivator is that cloud computing is _extremely_ expensive. If
an organization is genuinely only "paying for what you use," as all cloud
providers pitch, then a language that is twice as fast as another could result
in a massive decrease in monthly cloud charges. That is a performance
difference that would not have excited anyone fifteen years ago, but today, the
bean counters start paying attention to what the geeks are saying.

And so we have Rust. Or perhaps I should say so we have the _popularity_ of
Rust. Rust was going to be created regardless of the market, as is the wont of
geeks, but it is the specifics of our sociocultural age, our techno-mores, that
has allowed Rust to reach a surprising level of recognition and interest. Rust
has even achieved what no other language could in over twenty-five years: be
included in Linux's codebase. It has done all of this while being an arguably
academic language. Much like OCaml, its few concessions are related to operating
in the "real world," and the language makes them grudgingly, putting many of
them behind an explicit "unsafe" flag. Rust has learned all of the mistakes from
the past and set out not to repeat them, and we finally live in a world where
people in power care about that.

Rust is not alone in this shift,<sup>[[11](#fn11)]</sup> nor do I think that
Rust is the end goal of some grand historical arc. But for this moment, right
now, I think that Rust is part of the future, and it is a future that should be
encouraged. We as programmers are the midwives to a better world that struggles
to be born. We hold it now in our hands. Make it breathe.

--------------------------------------------------------------------------------

<a name="fn0">0</a>: "Any sufficiently complicated C or Fortran program
contains an ad hoc, informally-specified, bug-ridden, slow implementation of
half of Common Lisp."

<a name="fn1">1</a>: While _Design Patterns_ vis-a-vis languages is open to
significant critique, the concepts discussed in the book are still valuable,
especially as regards distributed systems. Solving problems of multi-service
systems necessarily cannot be solved with any current language and must be
solved in the implementation. The next great language may be one that turns
archiectural concerns into linguistic concerns, yet again obviating the book,
but for the time being, it remains a must-read.

<a name="fn2">2</a>: The ability of C to map to assembly in ways other
languages simply couldn't is best exemplified in the famous Duff's Device,
which literally just mashes together two syntax structures and somehow it
works. Look it up. It's wild.

<a name="fn3">3</a>: The narrative thread connecting Rust and C even goes back
to the very beginning of computing. The first people who could reasonably be
called programmers, Ada Lovelace and Charles Babbage, separated along the same
lines. Lovelace was a self-described metaphysician who recognized the
philosophical possibilities in Babbage's machine, while Babbage cared about the
machine itself and its ability to do mathematics.

<a name="fn4">4</a>: RIP Akira Toriyama. You were a brick in the wall of my
childhood.

<a name="fn5">5</a>: We had one glorious moment where an academic language
nearly achieved some degree of dominance in the 1980's with the rise of Lisp
Machines. Ironically, the two main Lisp Machine companies that emerged from MIT
left a drama so significant in their wake that it brought about the death of
the hacker culture at the university and was the direct cause for the modern
Open Source movement.

<a name="fn6">6</a>: We had another moment where a rigidly defined language
achieved huge success in the form of Ada. Sadly, real-world considerations
again killed Ada as its compiler, which needed to verify its powerful type
system, was cripplingly slow. Compilers back in the 80's were usually
proprietary products, so vendors of compilers would concentrate efforts where
there was money to be made. If Ada had come out during the era of online
open-source, who knows what could have been.

<a name="fn7">7</a>: Even though I don't consider them academic in the sense
that I defined, both Basic and Smalltalk emerged from academic settings. Their
aim was education of people, and they succeeded while becoming popular as well.

<a name="fn8">8</a>: Indeed, JavaScript gives us perhaps the best example of
this dynamic. An academic was given the task of creating a language for the web
and he set out to create an academic language, namely a variant of Scheme. This
plan was torpedoed by executives at Netscape who demanded that he instead make
a language like Java simply because Java was popular. And also call it
JavaScript, even though the languages are _completely_ different.

<a name="fn9">9</a>: The examples of this are too numerous to list, but some
glaring examples are Toyota's unintended acceleration debacle, the Therac 25,
the Boeing 737 MAX, Tesla's Autopilot, and the countless industrial accidents
involving manufacturing robots.

<a name="fn10">10</a>: Deloitte and its multiple lawsuits over its disastrous
failures to develop municipal services like DMV systems, or the famous dumpster
fire that was the launch of Obamacare.

<a name="fn11">11</a>: The most notable recent event being driven by similar
motivators is the rise of alternative server-side JavaScript runtimes like
LLVR, JustJS, Bun, and uWebSockets. Node spent over a decade focusing on
features and stability while its child, Deno, did similar in an attempt to
steal market share. Then Bun, which doesn't even support Windows yet, has blown
both out of the water in mindshare.

--------------------------------------------------------------------------------

The below text is valid Rust code. It is a copy of the code in the /src directory.

This was written in VSCode and using an interactive IDE with code linting,
folding, and highlighting is recommended.

``` rust
use std::ops::{Add, Sub};
/* These imports should be familiar to most. The double-colon syntax represents
the "path" to the entity. */
use std::{thread};
use futures::*;
use tokio::*;
use async_stream::stream;
use rand::prelude::*;
use std::rc::Rc;
use std::sync::{Arc, Mutex, mpsc};
use std::collections::HashMap;

// use futures_util::pin_mut;
// use futures_util::stream::StreamExt;

const _GREETING: &str = "Stay awhile. Stay forever.";

/* Comment blocks start with slash-star,
   and end with star-slash. */
// Line comments begin with a double slash.

/*** A note on the stack and heap: ***/

/* If you are coming from C or C++, the concepts of the stack and the heap are
familiar and your knowledge perfectly translates to Rust. If you are coming from
Go or TypeScript/JavaScript, some explanation may be required.

Data is stored in memory in two ways. The first is called the "stack" and the
name hints to its design. When a program or function is started, it gets a hunk
of memory given to it. This memory space is clear, contiguous, and entirely
owned by the program or function. The compiler knows how much memory space the
stack will require based its analysis of the code. For example, if a function
has five 32-bit integers in it, the compiler only reserves enough space in
memory to store five 32-bit integers. When the program or function is complete,
the memory is cleared. One of the most common memory errors is when the program
does something that exceeds the stack that was allocated, resulting in the
famous _stack overflow_, from which the website gets its name. Entities on the
stack are added to the "top" and removed from top in a "last-in-first-out", aka
LIFO, pattern. Imagine plates stacked; you always take the top plate.

The heap is precisely that: a big pile of memory space. The primary
differentiator between stack and heap entities is that anything on the stack
must be of known and fixed size. Anything that can change in size must exist on
the heap. The dangers of the heap include classics like null pointers and
memory leaks. Since the heap's structure is unknown, any access to an entity
requires an address.

There are significant performance implications in the stack versus the heap, but
these are outside the scope of this tutorial. If you are coming from C or C++,
these considerations are not new to you. If you are coming from TypeScript, then
don't even worry about it. Rust written in even the most naive way is still an
order of magnitude faster that JavaScript. If you are debating using Rust over
Go, on the other hand, be aware that naive Rust in the realm of n-tier
applications can often be _slower_ than naive Go. */

/*** A note on macros ***/

/* Throughtout this tutorial, you will see some commands appended with an
exclamation mark, like `println!`. This mark indicates that this command is a
"macro". Macros are _old_ in programming, having been first used in the 50's
and added to Lisp in 1963. A macro is something that evaluates a string of
tokens and then interprets it at compile time. A macro, when compiled,
"expands" into a larger amount of code. You can think of a macro as a compiler
that transforms provided glyphs into an implementation. A programmer could
genuinely implement their own programming language within a macro. Macros are
one of the constructs in Rust that most programmers are not likely to have
encountered. Macros will be discussed near the end of this tutorial. For the
time being, just be aware that the exclamation mark syntax indicates a macro. */

/*** A note on "unsafe" Rust ***/

/* As stated, Rust's design started in the symbolic realm and only moved as
close to the machine as was necessary for performance. Sometimes, though, when
maximizing performance, the best method is to abandon symbols and simply
manipulate the machine state directly like is possible in C and C++. Rust
allows you to do this but requires an explicit "unsafe" block. There are many
valid uses for unsafe code and Rust's best practices describe ways to mitigate
risks inherent in its use. If you are looking at high-performance application
development, then unsafe Rust is worth learning. I am mostly targeting n-tier
application developers and JavaScript engineers with this tutorial, though, and
thus consider unsafe Rust to be outside its scope. Read the official Rust docs
for more information. */


/*----------------------------------------------
* Cargo & Crates
*----------------------------------------------
*/

/*** Cargo ***/

/* Cargo is Rust's standard package manager/build tool. Conceptually it is
identical to NPM. Most Rust applications will be tested, built, published, and
have their dependencies managed with Cargo. Chances are, most new projects will
be bootstrapped with it as well. Cargo uses Crates.io as its standard publish
target, but just as with NPM, you can easily set up private registries. The
standard compiler used by Cargo is rustc.

Cargo uses the Cargo.toml file instead of package.json, but again, they are
extremely similar. And yes, Cargo uses yet another goddamned markup language.
And just like you, this was the first I heard of .toml files. I feel like I
should make my own markup language because everyone seems to be doing it. I'll
call it BLAML.

Cargo assumes a standard file structure. The Cargo.toml and Cargo.lock files
are at the root of a project. The src/ directory contains the main.rs file
which contains the main() function. The benches/ directory contains performance
tests. And the tests/ directory contains... tests. Specifically integration
tests.

Similar to newer JavaScript package managers and Go, Cargo centralizes
downloaded dependencies in its "home" directory. As such, you will likely only
ever have to download a dependency once.

You will interact with Cargo primarily through the basic Cargo commands:

`cargo` : downloads dependencies in the Cargo.toml file.
`cargo new your-project-name` : Bootstraps a Rust project.
`cargo add package-name` : Adds the package to the Cargo.toml and downloads it.
`cargo build` : Downloads any missing dependencies and builds the project.
`cargo run` : Builds _and_ executes your program.
`cargo test` : Rust functions labeled as tests.

There are many other commands and configurations. See the Cargo docs.


/*** Crates ***/

A crate is a unit of compilation. That is to say that a crate is a tree of
modules that is collapsed down to a single "thing." There are two types of
crates: binary and library. Binary crates are what an application is, namely a
tree of modules that gets turned into a binary executable. Library crates do
not have a main file and thus do not compile into a binary. They instead
compile into behaviors that can be used by other libraries or binary
applications. Most usage of the term "crate" is referring to library crates. In
that sense, if coming from JavaScript, you can think of a crate as an NPM
package. */


/*----------------------------------------------
* The Main Function
*----------------------------------------------
*/

/* The below is the main function which all Rust applications have. This quick
guide does not cover the structure of Rust applications. It focuses on the
syntax and semantics. For the purposes of this guide, it is sufficient to know
that Rust applications have a main function.

The syntax structure above the function is called an attribute. They allow a
developer to specify how the function is the be handled by the compiler. In this
case specifically the attribute specifies that this is the root function for the
async runtime Tokio. Tokio and async in general are discussed later.

The tutorial is structured such that the main function calls supporting
functions that each explain an element of Rust. This allows usage of an IDE
like VSCode to collapse the supporting functions to make scrolling easier and
faster while also using the function calls to jump to the function definitions
like a table of contents. */

#[tokio::main]
async fn main() {
    atributes();
    items();
    variables_and_bindings();
    ownership_and_borrowing();
    generics();
    primitive_types();
    type_structures();
    lifetimes();
    pattern_matching();
    opaque_types();
    smart_pointers();
    modules_and_crates();
    basic_operators();
    functions();
    multithreading_and_concurrency();
    async_syntax().await;
    macros();
    actix_web();
    rustdoc();
}

#[allow(dead_code)]
fn atributes() {
    /*----------------------------------------------
    * Attributes
    *----------------------------------------------
    */

    /* Attributes, as seen at the top of the various functions in this
    tutorial, are simply metadata for the compiler. They let you ignore certain
    errors, transform code, generate code, communicate with 3rd party tools, or
    enable features that are not active by default. They cannot be used to
    break the type system or suppress errors. Many attributes will be shown in
    this tutorial.
    
    There are two forms of attributes: outer and inner. Outer attributes are
    like the example above attributes(). They affect the thing that they are
    declared directly before. Inner attributes affect the thing in which they
    exist. Inner attributes do not work lexically. They apply to the entire
    entity in which they are declared. The primary use of inner attributes is
    for being declared at the top of a module or file, thus affecting all of
    its members. */

    /* The below applies to all children of attributes(). It could have easily
    been declared as an outer attribute as well. */
    #![allow(unused_variables)]
    
    /* Attributes in Rust have similar abilities to some code hygiene and
    linting tools. The standard linting implementation is called Clippy in a
    cheeky nod to Microsoft's gone-but-not-forgotten Office helper. Custom
    lints can be developed. */

    #[allow(non_camel_case_types)] // Only affects the line below.
    type the_answer = i32; 

    /* Attributes can denote deprecated functionality. When compiled, warnings
    will appear whenever deprecated code is called or otherwise used. */

    #[deprecated]
    fn deprecated_function() {
        println!("This is deprecated")
    }

    // This will throw a warning and have a visual strike-through in IDEs.
    deprecated_function();

    /* Attributes are used to denote functions that are tests. This allows easy
    co-location of tests with their implementations. Testing will be discussed
    in a dedicated section.
    
    Code generation via attributes takes two forms: trait derives and macros.
    Both of these are discussed more fully later. */
}

fn items() {
    /*----------------------------------------------
    * Items
    *----------------------------------------------
    */
  
    /* Items are entities that, whenever they are declared, they are analyzed
    and made global, even though their _visibility_ is restricted to the scope
    in which they were declared. They are "attached" to this scope like a key
    is attached to an object. This means that items can be referenced before
    they are declared. This is possible because items are entirely determined
    at compile time, meaning they exist before something like a function runs.
    In Rust parlance, items are "static" entities, with "dynamic" entities
    being their counterpart.
    
    The below is not a complete list of items, but represent the items
    important for this tutorial. All of these items will be discussed.
    
    - Constants
    - Enums
    - Unions
    - Function pointers
    - Implementations
    - Modules
    - Statics
    - Structs
    - Traits
    - Type aliases

    From the perspective of semantics, items are the "hard" pieces of Rust
    code, the things that represent the structure through which the logic
    flows. As such, items cannot be created dynamically. To continue this
    analogy, if we liken a program to a building, what happens in the building
    can change over time, but what happens in the building should not determine
    how many floors the building has. */
}

#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(unused_assignments)]
#[allow(dead_code)]
fn variables_and_bindings() {
    /*----------------------------------------------
    * Variables and Bindings
    *-----------------------------------------------
    */

    /*** Let ***/

    /* In the tradition of the ML language family that is Rust's inspiration,
    variables are not, by default, variable; they are bindings. As such, even
    though the term "variable" is commonly used, the more accurate term is
    "identifier." Thus, values are bound to identifiers, and only through
    identifiers can values be accessed (usually).
    
    Unlike the more pure, academic languages of OCaml and Haskell, Rust makes
    concessions to real-world performance. Making values mutable increases
    performance. Rust's compromise is to make values immutable by default. */

    let immutable_value = 42;

    /* NOTE: The above binding is an "evaluation," meaning the right-hand side
    is an "expression." Expressions are code that return a value. "Statements"
    are any valid and complete arrangement of symbols. The majority of Rust
    code statements are expressions. The above is identical to: */

    let immutable_value = { 42 };

    /* The above code is short but expressions can be long and have multiple
    sub-expressions and even side-effects. All that matters is that a value is
    ultimately returned.
    
    This also illustrates a concept in Rust called variable shadowing. This is
    far from unique to Rust but allows an engineer to re-declare a variable and
    have the new value apply henceforth. Shadowing allows the type to be
    changed because, even though the identifier name is the same, it is
    actually a _different_ identifier, which is why we can even re-declare the
    value as mutable. */

    // A mutable value is created with the `mut` modifier.
    let mut immutable_value = 2001;

    // New values must still abide by the type of the original binding.
    let mut mutable_value = 42;
    mutable_value = 2001;

    // Let identifiers can be initially unbound then bound later.
    let an_unbound_identifier;
    an_unbound_identifier = 1984;

    /* At this point, it is important to discuss the relationship of values and
    identifiers. Both values and identifiers are entities existing in memory. It
    is the identifier that determines how the program will interact with the
    value. Values are not inherently mutable or immutable (again, they are
    just 1's and 0's in memory somewhere) it is whether the `mut` modifier is
    present that determines if the program will _treat_ that value as mutable or
    immutable.
    
    This may seem like an esoteric restatement of what was already said, but it
    is important to understand this relationship to better grasp the later
    discussion of Rust's party piece, ownership. */


    /*** Constants ***/

    /* Rust's second method for declaring bindings is `const` for constant. If
    coming from other languages, `const` sadly yet again means something
    different in Rust. Like standard `let` bindings, `const` values cannot be
    changed, but they also cannot be modified as mutable. Constants must have a
    type annotation. Unlike `let`, constants can also be declared in any scope,
    including the global scope. You can see a constant at the top of this file.
    Constant identifier names use all capital letters as convention. Finally,
    Constants must be the result of an expression that can be determined at
    compile time. Constants cannot store a value that requires the program to
    run to determine. If you are coming from C, constants should be very
    familiar, only unlike C, the all-cap format is enforced. */

    const HALF_THE_ANSWER: i32 = THE_ANSWER / 2;
    const THE_ANSWER: i32 = 42;
    // const RANDOM_NUMBER: i32 = generate_random_int(); // Fails.

    /* Pay special attention to the constants HALF_THE_ANSWER and THE_ANSWER.
    Notice how THE_ANSWER is referenced _before_ it is declared. Constants are
    visible to everything within their scope, regardless of where they appear
    lexically. This is because they are one of the earlier-mentioned "items."
    Since all items are created in global memory, they can be referenced
    anywhere within the scope in which they are visible. */

    /* Constants cannot be shadowed within the same scope, but they can be
    shadowed in nested scopes. */

    // const THE_ANSWER: f64 = 3.14; // This does not work.

    let wrapper_scope = {
        const THE_ANSWER: f64 = 3.14; // This does work.
    };

    /* Constants provide a high-performance method for storing data of known,
    fixed size for sharing across parts of the application. */


    /*** Statics ***/

    /* Statics are the third and final way to declare a value in Rust. Static
    values are very similar to constants and many of the same rules apply. The
    difference is that a constant represents a value, while a static represents
    a memory location. As such, a key difference is that statics can be tagged
    as mutable. Rust requires any interactions with a mutable static to be
    flagged as unsafe since the value at that memory address can change
    unpredictably.
    
    By and large, constants will be used far more often than statics. The
    primary use case for statics over constants is when large amounts of data
    is being referenced. Constants are "inlined" during compilation. This means
    that everywhere that a constant is referenced, the value behind that
    constant replaces the reference. If the constant represents a lot of data,
    or if the constant is referenced many times, that could cause a huge
    increase in the size of the compiled binary. Statics put the data in one
    location only. */

    const CONST_VALUE: i32 = 42;
    let const_copy = CONST_VALUE; // This is a copy of the value 42.

    static mut STATIC_VALUE: i32 = 42;
    unsafe {
        let static_copy = STATIC_VALUE; // This is a copy of a memory address.
    }


    /*** Type Inference ***/

    // Rust can often infer types.

    let x = 42;                // 32-bit unsigned integer.
    let y = 3.14;              // 64-bit float.
    let z = "Dinner for one"; // &str, a primitive string of fixed size.

    // Functions cannot infer parameter and return types.
    fn add_ints(a: i32, b: i32) -> i32 { a + b }


    /*** Scope ***/

    /* Scope is the "space" in which entities exist and can thus be accessed.
    Scope should be pretty familiar to just about all programmers, but it is
    especially important in Rust because of how critical it is for automatic
    memory management. */

    // Binding scopes are delineated by curly braces.
    if true {
        let val1 = "best of times";
        println!("{}", val1)
    };

    let my_block = {
        let val1 = "worst of times";
        println!("{}", val1)
    };

    // You can even create naked blocks of scope.
    {
        let scoped_value = 2001;
    }

    /* Blocks are expressions, meaning that the last statement in the block is
    implicitly returned. Notice how a semicolon defines the termination of an
    evaluation block, meaning that no semicolon indicates the final return. */

    let my_block_value = { // Is set to 42.
        let x = 20;
        let y = 22;
        x + y // returns x + y
    };

    /* The list of reserved words can be found here:
    https://doc.rust-lang.org/reference/keywords.html

    Prefixing a variable name with an underscore creates a casual variable.
    These variables, if unused, will not trigger a compiler warning.
    
    Unused variable warnings only apply to block scopes and will not be
    raised on variables declared at the global or module level. 
    
    There are two kinds of unused variable. A "suspicious" unused variable is
    one that has been bound with `let`. An "innocuous" variable is one that has
    not.*/

    // To see the unused variable warnings below, comment out
    // #[allow(unused_variables)] near the top of this function.
    let block_scope = {
        let variable = 42;           // This triggers a warning.
        let _casual_variable = 2001; // This does not trigger a warning.
        "Return string"
    };


    /*** If ***/

    /* Since if blocks are expressions, they can be bound to an identifier. */

    let roll = 13;

    let crit = if roll > 10 {
        println!("Critical Hit!");
        true
    } else {
        println!("Miss!");
        false
    };


    /*** Destructuring ***/

    /* Rust allows destructuring bindings, where identifiers are written in the
    same structure as the value. This pattern is increasingly common and should
    be very familiar if you are coming from almost anything save for C. */

    let (first_value, second_value) = (2001, 42);

    println!("{first_value}"); // Prints 2001
    println!("{second_value}"); // Prints 42

    // This ignores the first value.
    let (_, only_value) = (2001, 42);

    /* This assigns the identifiers to the indices, starting with index 0, it
    then ignores everything until the final two values, where it saves the
    penultimate index and ignores the last. */
    let [index_0, .., index_6, _] = [0, 1, 2, 3, 4, 5, 6, 7];

    println!("{index_0}"); // Prints 0
    println!("{index_6}"); // Prints 6

    // Values can be destructured from structs.
    struct CustomStruct {
        id: String,
        value: i32,
    }

    let custom_value = CustomStruct{
        id: String::from("the_answer"),
        value: 42,
    };

    let CustomStruct{id: x, value: y} = custom_value;

    println!("{x}"); // Prints "the_answer"
    println!("{y}"); // Prints 42


    /*** References ***/

    /* As a memory managed language, how data is passed around is a key part of
    managing that memory. For those coming from TypeScript, this will seem like
    a new concept, but it's not _that_ new.
    
    A reference is simply a pointer to something. If coming from TypeScript, you
    are aware of the danger with passing an object or array to a function, since
    if you mutate that object or array inside of that function, it gets changed
    _outside_ of the function. That is because what you passed into the function
    was a reference to the object or array, not the object or array itself.
    
    The only major difference in Rust is that an engineer can explicitly
    reference _any_ value, not just objects or arrays, by prepending an
    ampersand to the value name. This is not a surprise to C or Go developers,
    so you can skip ahead. */

    let a_basic_number = 42; // type of i32
    let another_number = a_basic_number; // another i32
    let a_basic_reference = &a_basic_number; // type of &i32

    /* In the above, `a_basic_number` is a 32-bit integer. When aliasing
    primitives, they are copied by default, so `another_number` is also a 32-bit
    integer. But if an ampersand is prepended to the variable usage, it becomes
    a reference that holds no value, but instead points to the location of
    another value. The copy does not occur.

    This default copy behavior is only true for values that exist on the stack.
    For heap-allocated values, such as vectors, copying must be explicitly
    performed. This should be familiar to JavaScript developers who have used
    libraries such as ImmutableJS to prevent sharing of arrays and objects.
    Vectors will be covered later, but for now, just know that they are very
    similar to JavaScript arrays and they exist on the heap. If you did not
    already know, all values in JavaScript exist on the heap; there is no
    stack. */

    let a_vector = vec![1, 2, 3, 5, 7];
    let not_another_vector = a_vector;
    // let still_not_another_vector = a_vector; // This fails

    /* In the above, the identifier `not_another_vector` is not bound to a copy
    of `a_vector`, it has instead been bound to the reference that points to
    the value `[1, 2, 3, 5, 7]`. In Rust, this means that `a_vector` is no
    longer a valid identifier. This part is not familiar to JavaScript
    developers but is a key part of Rust's safety system, that is to say,
    "ownership." Ownership is Rust's party piece. Both it, and what happened
    above, will be discussed in a dedicated section. */


    /*** Pointers ***/

    /* As was mentioned earlier, only through identifiers can values be accessed
    and mutated... usually. Considering that Rust is inspired by C, all of you
    C developers were probably immediately thinking about _pointers_.
    
    A pointer is also a form of identifier, but instead of identifying something
    in the symbolic structure of the program, it identifies something in the
    computer itself. This usually means that the pointer "points" to a specific
    location in the computer's heap memory.
    
    This form of value access is extremely performant since it directly uses the
    computer instead of passing through the symbolic realm. But it is also very
    dangerous. This is why pointer usage is associated with what is known as
    "unsafe" Rust.
    
    Except for library writing, unsafe Rust is something that the average
    developer will never do. "Safe" Rust performance is already so good that
    unsafe Rust is unnecessary in all but the most demanding situations.

    There is a lot of unsafe geekery online and it is best to ignore this.
    Unsafe Rust can indeed be more performant, but it requires stepping outside
    of what makes Rust Rust. Further, most use cases where unsafe code is
    genuinely best are also very common, low-level operations, like swapping
    bytes in memory. These common use cases almost always have a module in the
    standard library.
    
    For the time being, a cursory explanation of pointers will suffice.

    The most common way to create a pointer is to bind the memory location
    of a reference. This is done through a new type signature that will be
    familiar to C developers, the asterisk. */

    let a_number_in_memory: i32 = 42;

    // Creating a pointer is safe.
    let a_pointer_to_a_number: *const i32 = &a_number_in_memory;

    unsafe{
        // Only consuming a pointer is unsafe.
        let pointer_to_a_new_number = a_pointer_to_a_number.add(42);
    }

    /* Rust also enables direct memory access via memory addresses. This is
    perhaps _maximally_ usafe Rust and is thus far outside the scope of this
    tutorial. */

}

#[allow(unused_variables)]
fn ownership_and_borrowing() {
    /*----------------------------------------------
    * Ownership & Borrowing
    *----------------------------------------------
    */

    /* In Rust, every value has an "owner". An owner is also known as an
    identifier since only through the identifier can a value be accessed. When
    an owner goes out of scope, such as when a function completes, the value is
    "dropped" from memory. Rust does this automatically, but also supports an
    explicit "drop" command. Again, if coming from C++, all of this should be
    familiar through what is known as RAII, but unlike C++, Rust does this
    automaically and by default. If you write good, simple Rust, you will very
    likely never have to concern yourself with the cleanup process. C and C++
    engineers will like that, but it is TypeScript engineers who should take
    most note, because it means that Rust can _feel_ garbage collected, making
    it much more approachable. That said, Rust's system of ownership can be
    restricting, confusing, and the compile errors that it produces can
    sometimes seem strange.
    
    But before that, let us go over the basics. */

    let catcher_in_the_rust = "Holden Caulfield";

    /* In the above, the value of "Holden Caulfield" is owned by the entity
    `catcher_in_the_rust`. They are "bound". That means that
    `catcher_in_the_rust` _owns_ that value. The value is a string literal,
    meaning a sequence of chars of known length, hard-coded into the binary,
    and which sits on the stack and not the heap. In this scenario, "Holden
    Caulfield" exists on the stack. It is on the unpredictable heap where
    things get interesting. */

    /* Using the String crate from the standard library allows us to create a
    string of unknown size on the heap. By unknown, I mean that the compiler
    does not know how long the string will be at compile time. It can only be
    known for sure during runtime. */

    let catcher_in_the_string = String::from(catcher_in_the_rust);

    /* The identifier `catcher_in_the_string` now stores a pointer to a
    location on the heap which contains the string "Holden Caulfield". So here,
    things get interesting. */

    let catcher_in_the_stack = catcher_in_the_rust;
    let catcher_in_the_heap = catcher_in_the_string;

    /* Both of these actions are copying from the previous variables, but _what_
    is being copied is very different. `catcher_in_the_rust` is on the stack,
    as such the string "Holden Caulfield" is completely copied and a new
    instance of it is placed on the stack with the identifier
    `catcher_in_the_stack`. `catcher_in_the_heap`, meanwhile, is copying the
    _pointer_ to the value "Holden Caulfield", which exists somewhere on the
    heap.
    
    So in the first scenario, a new value is created, but in the second, there
    is only one value with two identifiers pointing to it. This can be dangerous
    because a programmer could interact with both identifiers without realizing
    that they point to the same data. Rust manages this problem with ownership.
    Now that `catcher_in_the_heap` exists, it "moves" the value from
    `catcher_in_the_string`. As such, a programmer is not allowed to interact
    with the previous identifier. See below. */

    // let attempted_move = catcher_in_the_string;

    /* If you uncomment the above code, it will throw an error saying that the
    value `catcher_in_the_string` has "moved". It has indeed. The identifier
    that now owns that value is instead `catcher_in_the_heap`. As such, this
    nearly identical line succeeds. */

    let successful_move = catcher_in_the_heap;

    /* Now, `successful_move` owns the value and trying to reference 
    `catcher_in_the_heap` again would fail.

    Ownership tracking is Rust's safety net. Because remember, Rust clears
    memory when an identifier goes out of scope. See below. */

    let thats_what_i_want = String::from("Gimme money!");

    {
        let new_owner = thats_what_i_want;
        // After this, `new_owner` falls out of scope, and thus its memory is
        // cleared and the value dropped.
    }

    /* From this point forward, the value "Gimme money!" no longer exists
    anywhere in memory. Rust tracks this and will throw an error if the below
    line is uncommented. */
    
    // let money = thats_what_i_want;

    /* This scenario extends beyond naked blocks and variable aliasing. At any
    point where a heap value is moved, ownership and its associated memory
    cleanup will occur. */

    let istanbul = String::from("was Constantinople.");

    fn memory_destroyer(x: String) {
        println!("{}", x);
        // x is now dropped.
    }

    memory_destroyer(istanbul);
    // "was Constantinople." is now dropped.

    // Thus this will not work.
    // println!("{}", istanbul);

    /* In the above, `x` could theoretically be returned, and thus have
    ownership transferred back out of the function, but this is an unwieldy
    pattern. To allow values to be used without transferring ownership, Rust
    leverages the concept of "borrowing".
   
    This process is called borrowing because ownership is not transferred but
    only one entity can borrow a value at a time. The below is identical to the
    previous naked scope. */

    let respect = String::from("Find out what it means to me.");

    {
        let new_borrower = &respect;
        /* Note the ampersand before the value, indicating that this is a
        reference. After this, only the reference is destroyed. The value is
        untouched. Because it is only a reference, multiple aliases to the
        original value are possible. */
        let another_new_borrower = &respect;
    }

    /* A function works the same way. */

    fn borrower(x: &str) {
        println!("{}", x);
    }

    borrower(&respect);
    // "Find out what it means to me." is not dropped.

    /* Just as normal variable declarations are immutable by default, so are
    references. References have two layers of protection in that both the
    original value _and_ the reference must be tagged as mutable if the value
    is to be changed. When mutable, references are treated more carefully by
    the compiler. Any number of references can be _created_, but only one
    reference can be _consumed_. Rust does not call it this, but I call this a
    "strict" borrow in the sense that if someone borrows something, no one else
    can borrow it until it has been returned. Rust simply calls this a mutable
    borrow. I prefer higher-level terminology for linguistically fundamental
    concepts as this, though. As such, immutable borrows as mentioned
    previously are "casual" borrows, while mutable borrows are "strict". See
    below.*/

    let mut jeremiah = String::from("was a bullfrog.");

    {
        let new_borrower = &mut jeremiah;
        let another_new_borrower = &mut jeremiah;
        let yet_another_new_borrower = &mut jeremiah;
        // println!("{}", new_borrower) // Throws borrow error

        // Meanwhile this succeeds because it was the most recent borrow.
        println!("{}", yet_another_new_borrower);
    }

    /* The naked scope is now closed and all references are dropped. We can now
    freely borrow `jeremiah` again. */

    let new_borrower = &mut jeremiah;
    println!("{}", new_borrower);

    /* The mechanism performing these checks is called the "borrow checker." The
    point of the borrow checker is to prevent unexpected changes to values while
    the program runs. For those coming from something more free-wheeling and
    anarchic like JavaScript, this can initially feel overly restrictive, but
    it is _critical_ to Rust's value. Whole classes of errors are eliminated by
    this semantic decision. Learn it. Live it. Love it. */
}

#[allow(dead_code)]
fn generics() {
    /*----------------------------------------------
    * Generics
    *-----------------------------------------------
    */

    /* If you are coming from C++ or TypeScript, generics should be very
    familiar to you. Go only recently introduced them, but they are common
    across most typed languages. Generics are essentially just entities that
    can accept different types. The syntax for this is broadly similar to
    TypeScript, so it should be easy to pick up. */

    struct Things<T1, T2> {
        thing1: T1,
        thing2: T2,
    }
}

#[allow(unused_variables)]
fn primitive_types() {
    /*----------------------------------------------
    * Primitive Types
    *-----------------------------------------------
    */

    /*** Integer ***/

    let val1: i32 = 1 + 1; // i32 = 2

    // Integer division will round results
    let val2 = 8 / 3; // i32 = 2
    let val3 = 8 / 5; // i32 = 1

    /* All integers in Rust are 32-bit signed integers by default, as you can
    see above. Rust supports 8, 16, 32, 64, and 128-bit signed and unsigned
    integers. No current architectures support 128-bit integers natively, so
    usage of it can be dangerous for performance.
    
    Being a lower-level language, Rust also supports explicit support for
    per-architecture typing of both signed and unsigned integers. This means
    that using these types on a 64-bit Windows system will result in 64-bit
    integers. I am writing this on a Samsung Windows laptop, meaning these
    integers below are 64-bit on my system. */

    let arch_s_int: isize = 42;
    let arch_u_int: usize = 42;

    /*** Float ***/

    let float1 = 1.1 + 1.5;     // float = 2.6

    // Floats and integers can be formatted with underscores. These underscores
    // are purely visual.
    let formatted_int = 12_34_56;       // i32 = 123456
    let formatted_float = 12_34_56.0;   // f64 = 123456.0

    /*** Char ***/

    /* Char represents Rust's true text primitive. Strings are best thought of
    as an array or vector of chars. Rust supports unicode and as such a char is
    actually a scalar value. In code, a char can be represented either as a
    glyph or as the scalar values as found on the Wikipedia page for Unicode
    characters: https://en.wikipedia.org/wiki/List_of_Unicode_characters
    
    Be careful, though, because treating a glyph like a char is not always
    correct. There are many singular glyphs that actually require multiple chars
    to represent. This is primarily a concern when converting from a string. */

    // Use single quotes for a char.
    let last_letter = 'z';
    let last_letter_unicode = '\u{007a}';
    let kannada_char = 'ಠ';
    let kannada_char_unicode = '\u{0ca0}';


    /*** Array ***/

    /* If you are coming from C, Java, or Go, then arrays in Rust will be
    immediately familiar. If you are coming from TypeScript or JavaScript, they
    will be a little different. Arrays in Rust are typed, fixed-length lists of
    entities. Unlike TypeScript, you cannot simply push to an array because an
    array's length must be set when it is instantiated. JavaScript runtimes hide
    this complexity from you when using arrays. */

    // The length of an array is part of the type.
    let array_of_five: [i32; 5] = [1, 2, 3, 42, 5];

    // Arrays cannot be instantied with no values. Unless the length is
    // specified as zero. An element type is still required.
    let array_of_none: [i32; 0] = [];

    // Arrays can be initialized with the same value in all indices, though.
    let array_of_ten: [i32; 10] = [42; 10];

    /* The value syntax above is simply saying to create an array with the value
    of 42 set in 10 indices. */

    // Arrays are index accessed.
    let the_number_3 = array_of_five[2];

    // Array lengths can obviously not be changed, but neither can values.
    // Arrays must be declared as mutable to do this.
    let mut mutable_array_of_five: [i32; 5] = [1, 2, 3, 42, 5];
    let value_from_mutable_array = mutable_array_of_five[2];
    mutable_array_of_five[2] = 314;

    /* When index accessed, primitive values are copied. As such, even though this
    index was changed in the above line, this value is still 3 and not 314. */
    println!("{}", value_from_mutable_array);

    // But this value is 314.
    println!("{}", mutable_array_of_five[2]);

    /* When values from arrays are borrowed, the entire array is locked. This is
    because when an array is dropped, so to are all its values. As far as memory
    is concerned, an array and its values are tied. */

    let borrowed_value_from_mutable_array = &mutable_array_of_five[2];
    // mutable_array_of_five[2] = 314; // Borrow error.
    println!("{}", borrowed_value_from_mutable_array);


    /*** Slice ***/

    /* Slices are the primary tool with which you will interact with arrays.
    Slices are very similar to slices in C++ but notably different than in Go.
    In Rust, a slice is simply a window into a preexisting sequence of data. For
    the sake of this section, that sequence of data is an array.

    Go's slices behave similarly to arrays in JavaScript or TypeScript, to wit
    a slice can be instantiated independently of an array. The Go runtime will
    instantiate an array behind the scenes that will grow or shrink based on the
    slice. This makes the developer experience nicer but has performance
    implications. Rust does not do this and a slice must have data behind it. */

    let base_array = [1, 2, 3, 4, 5];

    // Slices are created by providing bounds with indices.
    let slice_of_three = &base_array[1..4];

    // Slicing the entire array can be done by omitting the values.
    let slice_of_five = &base_array[..];

    /* Slices are more generic in Rust than other languages, and as such they
    can be used with more than just arrays. Most importantly, slices can be
    used with vectors and strings, which will be discussed now. */


    /*** Vector ***/

    /* For Go developers accustomed to slices and JavaScript/TypeScript
    developers accustomed to arrays, Rust has a "vector." Vector is not a true
    primitive and is instead part of the standard library. Vectors are typed
    lists, similar to arrays, but are dynamically sized and exist on the heap.
    The Rust documentation calls them a "collection." Vectors are essentially
    syntactic sugar over implementations and as such rely on "generics."
    Generics are common in other languages like TypeScript or C++, so this
    should not be too alien for anyone, but generics will also be discussed
    later. */

    // The generic type <i32> creates an empty vector that stores i32's.
    let mut viktor: Vec<i32> = Vec::new();

    // Vectors can infer types as well when created with the vector macro.
    let v = vec![42, 2001, 314, 1999];

    // Vectors behave similarly to JavaScript arrays.
    viktor.push(42);
    viktor.push(2001);
    viktor.push(314);
    viktor.pop();
    let the_answer = viktor[0];
    viktor.extend(&v);

    // Direct index access runs the risk of out-of-bounds errors.
    // let invalid_index = viktor[9]; // This compiles but will fail to run.

    // Using the get method safely returns an option.
    let invalid_index = viktor.get(9);

    // Just like arrays, borrowing an index from a vector will lock the vector.
    let index_1 = &viktor[1];
    // viktor.push(1999); // Causes borrowing error.
    println!("{}", index_1);

    /* Slice syntax on vectors is identical to arrays. */
    let rogue = &v[..];
    let johnny = &v[1..3];


    /*** String & str ***/

    /* Like vectors, strings in Rust are not true primitives in the sense that
    a primitive is a thing of known, fixed size. They are like C strings in that
    they are best described as an array of characters, with each character being
    the true primitive, and are again classified as a collection.
    
    The fundamental type, to wit the type that is part of the language itself,
    is `str`, which is a sequence of chars _somewhere_ in memory. It could be
    on the stack, it could be on the heap. What is important to know is that
    the compiler does not know the length of this sequence at compile time. As
    such, when creating str entities, they are almost always _referenced_. As
    such, they usually exist in slice form, to wit &str. The string slice does
    have a known length.
    
    To illustrate, "ABCDEFG" are chars sitting next to each other in memory. A
    slice can represent all of them, some of them, or none of them. Rust does
    not have a type for representing the sequence of chars itself. */

    // Use double quotes for strings.
    let greeting = "Hello world!"; // type of &str

    /* Notice how the type above is a _reference_. That is because, since Rust
    has no true string primitive, the identifier `greeting` is actually a
    reference to a collection of chars hard-coded into the binary and sitting
    on the stack. This means string slices never own the memory in which the
    chars exist. */

    // Strings can span multiple lines.
    let a_longer_greeting = "Look at me,
    I'm a
    multi-line string";

    /* The `String` type that was used in earlier examples to create a string
    on the heap is actually part of the standard library and is fundamentally a
    wrapper around `str` that provides helpful functionality. Because of the
    common usage of `String`, the two types are often confused in conversation,
    with people using the term "string" to refer to either `String` or `str`.
    
    To reiterate, using the String crate from the standard library creates a
    sequence of chars on the heap. This string can be added to and reduced, but
    as it is a collection, interactions with it are similar to a vector.
    Indeed, this is because under the covers, String _is_ a vector. */

    let mut heap_of_chars = String::from("A few of my favorite things: ");
    
    heap_of_chars.push_str("raindrops on roses, ");
    heap_of_chars.push_str("whiskers on kittens. ");

    let string_slice = &heap_of_chars[1..3];

    /* String can be coerced into &str via the type of a function parameter. */

    fn str_coercer(s: &str) {
        println!("{s}")
    }

    // In the below, a &String is being coerced into a &str.
    str_coercer(&heap_of_chars);


    /*** Hash Maps ***/

    /* Hash maps in Rust are conceptually similar to objects in JavaScript.
    Whereas vectors store data by index, hash maps store data by key. They
    exist on the heap and can grow and shrink dynamically. Unlike vectors, hash
    maps are not ambiently available like vectors and strings and must be
    imported to use. You can see the imported crate at the top of this file. */

    // Instantiate a map of default size.
    let mut inventory = HashMap::new();

    // Inserting accepts a key and value tuple. The hash map will take ownership
    // of the key and value where appropriate.
    inventory.insert("dune", 10);

    // Interacting with a map, since keys are unknown, uses options.

    // New values return the inserted value in Some().
    let new_value = inventory.insert("krull", 5); // Some(5)

    // Duplicate values return None.
    let dupe_value = inventory.insert("dune", 10); // None

    // Getting values also return an option.
    let krull_copies = inventory.get("krull"); // Some(&5)

    /* Make note of the reference in the above return value. Remember how all
    values from vectors must be references that lock the vector? The same is
    true for hash maps. Hash maps own their contents. */

}

#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
#[allow(unused_mut)]
fn type_structures() {
    /*----------------------------------------------
    * Type Structures
    *-----------------------------------------------
    */

    /* As I mentioned, Rust is heavily inspired by OCaml. It tries to bridge the
    conceptual gap between highly symbolic, academic languages like ML and
    hardware-oriented languages like C. This means that the linguistic
    traditions of both languages sit sometimes uneasily next to one another.
    
    An excellent example of this is the distinction between "types", as
    illustrated earlier, and structs, tuples, traits, interfaces, and
    implementations. In C, these are different things because they represent
    different things in memory. In OCaml, there is no significant difference;
    they are all just "types." A type is simply a description of the ways with
    which a thing can be interacted, and anything that fulfills that is simply
    declared with `type`.
    
    Rust documentation sometimes conflates these two traditions, calling things
    types in one place, then calling them structs or tuples in another. I will
    try to solidify my term usage going forward. Structs, tuples, enums, and
    traits are collectively called types because a type necessarily does not
    define *how* something is done, only *what* is done. As such, even though
    implementations add to the shape of a struct and thus alter its type
    signature, they are not truly an extension of the type per se. The type
    extension is incidental to the implementation. */


    /*** Structs ***/
    
    /* A struct is a grouping of entities under a single name and usually
    sitting together in memory. The terminology and symblic nature are from C,
    which itself took the concept from ALGOL.

    From the developer perspective, a struct is like a blueprint that is built
    when the struct is instantiated. For developers coming from TypeScript, it
    may look like a type or interface, but there are differences which will be
    discussed.
    
    The first, and most notable difference, is that structs can only contain
    data primitives. This is because, as mentioned, an instantiated struct
    represents data sitting together in memory. As such, the data must be of
    known, fixed size at the moment of instantiation. */

    // Idiomatic Rust uses CamelCase for struct declarations.
    struct UserData {
        id: String,
        name: String,
        display_name: String,
        bank_balance: f64,
    }

    /* Unlike TypeScript, but like C, a struct is not simply annotated on an
    object. The struct is instantiated explicitly, as though you are calling a
    function that returns an object. */

    let mut a_user = UserData {
        id: String::from("abc123"),
        name: String::from("Duncan Idaho"),
        display_name: String::from("The Duncanator"),
        bank_balance: 3.14,
    };

    /* Note how the above UserData instance is tagged as mutable. This allows
    changes to the internal values *and* the binding, but another instance of
    the same struct must be used. Individual fields of a struct cannot be
    independently tagged as mutable. */

    /* Rust does not allow un-typed structs. This will be one of the biggest
    differences for those coming from TypeScript. In TypeScript, this is
    perfectly acceptable:
    
    const anUntypedObject = {
        id: "abc123",
        name: "Duncan Idaho",
        display_name: "The Duncanator",
        bank_balance: 3.14,
    }

    TypeScript will infer an anonymous type that will then be used to type-check
    later uses of this object. This sort of entity in JavaScript and TypeScript
    is called an object literal. Rust also has the term "struct literal" but it
    denotes the entity created when instantiating a struct. */

    /*** Struct Updating ***/

    /* Structs can be partially copied with spread-like syntax that should be
    familiar to JavaScript developers. Be aware of ownership, though. `name`
    and `display_name` are now no longer valid on `a_user`. */

    let another_user = UserData {
        id: String::from("efg456"),
        ..a_user
    };


    /*** Unit Struct ***/

    /* One of the best aspects of OCaml, since it separated the existence of an
    identifier and value in both the logic and the type structure, was to have
    _unbound_ type identifiers that had no values. These are known as
    _abstract_ types. In Rust, the language is a bit muddled and they instead
    refer to them as "unit structs". A discussion of `unit` will come later.

    The below struct is a unit struct. Any entity using this struct does not
    need to know its structure just so long as usage of the type is consistent.
    Basically, programs can use symbols without knowing what they mean.
    
    The superpower of unit structs is to enable type-safe program structure
    experimentation and scaffolding. A developer can draw out a structure on a
    whiteboard, then code out the same structure, with unit structs being used
    to connect the entities. Later, when behaviors are figured out, the structs
    can be built out while maintaining strict type security across all of the
    struct consumers. 
    
    Unit structs will be explored in detail in the sections on enums and
    implementations. */

    struct Kwyjibo;


    /*** Type Aliases ***/

    /* Rust enables aliasing of types to different names. Aliases are
    confusingly declared with the `type` keyword. This is one of the few areas
    of Rust's syntax with which I strongly disagree. `type` is from OCaml and
    they should have left it there. Aliases are intended to enable semantic
    naming of broad, generic types. For example, below, a vector representing
    stops on a trip can have the type aliased so the type of the list itself
    provides semantic information. Now, regardless of the identifier used, it
    could even be the dreaded "data", semantic information about what the
    identifier represents is not lost. This is a powerful tool for self-
    documenting code and is common in TypeScript. */

    type JarJarBinksQuotes = Vec<String>;

    /* Type aliasing has a second use for creating entities called "opaque
    types." These are addressed later because they do not work at the moment. */


    /*** Tuples ***/

    /* In Rust, tuples are considered a form of struct. I disagree with this
    and think that it comes from the conflating of OCaml and C terminology. In
    Rust, "struct" is used in a broadly similar way to "type" in OCaml, as
    illustrated above with the unit struct, but because structs represent
    physical memory, the analogy necessarily breaks.
    
    Like a traditional struct, a tuple contains data packed together in memory,
    but unlike a struct, the constituents of the tuple are unnamed. Instead,
    they are defined by their order. */

    struct ATuple (String, i32);
    let the_answer = ATuple(String::from("The answer is "), 42);

    /* An important difference between structs and tuples, and a major reason
    why I think calling tuples a form of struct is a mistake, is that tuples can
    be instantiated _without_ a declared struct type. */
    
    let an_untyped_tuple = ("The answer is ", 42);

    /* The above will evaluate to an anonymous type of (&str, i32). There are
    good reasons for why tuples can be type-inferred like integers or strings
    while structs cannot. My criticism is not with the implementation. It is
    with the term usage because terms are important. */

    /* Tuples can be dot-accessed with a zero-based index, similar to arrays or
    lists in some other languages. */

    let the_tuple_says = an_untyped_tuple.1; // 42


    /*** Struct Methods ***/

    /* As mentioned, structs are just grouped data. They are not like classic
    "objects" in the sense of having behavior and data. To add behaviors, aka
    methods, to a struct, they must be "implemented". Implementations exist
    separate from the struct in memory. */

    struct Square {
        width: f64,
        height: f64,
    }

    impl Square {
        fn area(&self) -> f64 {
            self.height * self.width
        }
    }

    /* Implementations written in the same module as the struct, in the lib.rs
    file, or in the main.rs file are available globally. Implementations
    written in any other module are visible only in that module, thus allowing
    a module to use a struct and have private functionality tied to that
    struct. If importing a struct from another module, visibility of the
    contents of an implementation follow similar rules to modules, where
    methods need to be labeled as public if they are to be used outside of the
    scope in which they are declared. */

    
    /*** Traits ***/

    /* Traits are how Rust handles function signatures separate from their
    implementations. In other languages, a function signature can simply be
    added to a type or interface. In Rust they are separate. Traits are how
    shared behavior can be defined. For example, a for_each() method can be
    defined that can be implemented for any entity. The specific implementation
    for each entity type may be different, but they share the name and
    signature. */

    trait Extend {
        fn extend(&self, length: f64) -> f64;

        // Default implementations can be defined in a trait block.
        fn say_something(&self) -> String {
            // Default implementations can call other functions.
            let val = self.extend(42.0).to_string();
            String::from("{val}")
        }
    }

    impl Extend for Square {
        fn extend(&self, amount: f64) -> f64 {
            let area = self.area();
            area * amount
        }
        // Since say_something has a default implementation, it does not need to
        // be specified here.
    }

    let new_square = Square{
        width: 5.0,
        height: 5.0,
    };

    let vol = new_square.extend(4.0);
    println!("Volume is {vol}");

    /* Traits can be used as type parameters. This is discussed at greater
    length in the section on "Opaque Types." In the below function, the
    signature is saying that it returns _something_ that implements Extend. It
    returns a Square, which implements Extend, so that fulfills the signature.
    Multiple traits can be specified with the `+` operator, e.g. `impl Extend +
    ToString`. */

    fn get_extendable_thing() -> impl Extend {
        Square{
            width: 42.0,
            height: 42.0,
        }
    }

    /* NOTE: Notice how Rust's syntax lets you know what sort of entity is
    being called. Dot syntax indicates something that is attached to, for lack
    of a better word, an object. The double-colon syntax, like seen at the top
    of this file, indicates something that is a member of a module. A good
    comparison is String::from() for creating Strings and .to_string() which
    also creates strings. */


    /*** Supertraits ***/

    /* Traits can be tied to one another in a parent-child relationship,
    meaning that if the child is implemented, so to must the parent. */

    trait Vehicle {
        fn honk() -> String;
    }

    trait Engine: Vehicle {
        fn accelerate() -> i32;
    }

    /* In the above, if Engine is implemented, Vehicle must also be
    implemented. Just like trait type parameters, supertraits can contain
    multiple traits via the `+` operator, e.g. Engine: Vehicle + Transmission.
    */


    /*** Union ***/

    /* The reason for why Rust did not simply take OCaml's terminology is
    because Rust also took the union type from C. Unlike the enum, which can be
    understood separate from the machine state, a union requires an
    understanding of memory. A union is a section of memory that can store any
    of the value types as listed in the union. This means that the memory
    consumed upon instantiation will be whatever is required to fit the largest
    type in the union. */

    union IntUnion {
        small_int: i8,
        large_int: i64,
    }

    /* In the above union, 64 bits of space will be reserved because one
    possible value is a 64-bit integer.
    
    Chances are you will never use a union. Interacting with them is part of
    what is considered unsafe Rust and an enum will be a better choice in
    almost every case. The primary use case for unions is when extreme memory
    efficiency is needed and the safety of enums consumes memory. */


    /*** Enum ***/

    /* If you have any experience with OCaml, you will recognize `enum` as
    being equivalent to `union`, sometimes called discriminated union or tagged
    union. Enum is a type that defines an identifier that can be bound to any
    of the enum's consituent types. For example, a value could be a string _or_
    a 32-bit integer, an enum allows that to be represented. The Rust type
    checker will then ensure that any code that consumes an enum must handle
    all pertinent cases. */

    struct Ford;
    struct Toyota;
    struct BMW;

    enum Car {
        Ford,
        Toyota,
        BMW,
    }

    let my_car: Car = Car::BMW;

    /* Each possible state of an enum is called a variant, since they are the
    varied ways in which an enum can exist. The variants are distinguished from
    one another via the aforementioned tag or discriminant. The above Car enum
    has implicit discriminants, integers starting with 0. Thus, Ford is 0,
    Toyota is 1, and BMW is 2. This should be familiar to anyone coming from
    TypeScript, C, C++, or really any number of languages. Discriminants can be
    any integer. */

    enum Truck {
        F150 = 420,
        Silverado = 42,
        Tundra = 69,
    }

    /* Notice in the above enum how the three trucks were not declared as
    structs before. An enum necessarily contains other types, meaning that any
    variant that has not been previously declared is automatically declared as
    a unit struct. More complex type structures can be represented as well. */
    
    enum Motorcycle {
        Harley{exhaust: String},
        Ducati{color: String},
        Honda{has_vanilla_ice: bool},
    }

    /*** > Option ***/

    /* I originally had Option in the section on language primitives. That is
    how foundational it is to Rust's functioning. Option is discussed bere
    because it is actually an enum, but one so common it is included with the
    language itself.
    
    As mentioned, Rust was inspired by functional languages, and one great
    thing in them is the ability to reliably represent nothing, as with unit,
    and also the _possibility_ of something or nothing. Whenever an Option is
    used, what is actually passed is a "box" that either has the specified type
    or is empty. This pattern enforces robust null checks.
    
    Further, Rust does not support optional arguments as it is found in many
    other languages. Instead, Options are used to again enforce type safety.
    Implementing that requires pattern matching, which will be discussed
    shortly. */

    fn generate_answer() -> Option<i32> {
        if rand::random::<bool>() {
            Some(42)
        } else {
            None
        }
    }

    let possible_answer = generate_answer();

    if possible_answer == Some(42) {
        println!("The answer is 42")
    }

    /*** Result and Error Handling ***/

    /* Rust differentiates its error types between "recoverable" and
    "unrecoverable" errors. Recoverable errors mean that the symbolic state of
    the program is correct, something expected but undesriable has occured, and
    later control flows will direct the program into a state intended to handle
    the error. This form of error is handled with `Result`. Unrecoverable
    errors mean that the symbolic state of the system has been violated and as
    such there is no way to direct the program into a useful state. Thus, the
    program terminates.
    
    Result is another enum included in the language that serves a similar
    purpose to Option, but instead of Some/None, its containers are Ok/Err, for
    capturing a success or returning error information. This is Rust's
    structured way of handling errors as opposed to mechanisms like
    `throw/catch`. If you are coming from Go, you will notice some conceptual
    similarities in error handling, but while Go makes error handling optional,
    Rust enforces it.
    
    Rust also enforces a pattern that is simply considered "best practice" in
    other languages: no untyped errors. In languages such as JavaScript, it is
    common to simply log a string. Languages like Python encourage error
    objects, but their lax typing means that the actual error is often not
    captured by the error objects, resulting in actively inferior logging
    information. Rust's strict typing combined with error enforcement means
    errors will always be explanatory. */

    enum ResultError {
        ErrOne(String),
        ErrTwo(String),
    }

    type TestResult = Result<i32, ResultError>;

    fn generate_result() -> TestResult {
        if rand::random::<bool>() {
            Ok(42)
        } else {
            Err(ResultError::ErrTwo(String::from("There was no answer")))
        }
    }

    let possible_result = generate_result();

    /* NOTE: The value of enums like Option and Result will be discussed shortly
    in the section on "Pattern Matching". */
}

#[allow(unused_assignments)]
#[allow(unused_variables)]
#[allow(dead_code)]
fn lifetimes() {
    /*----------------------------------------------
    * Lifetimes
    *-----------------------------------------------
    */

    /* All entities in Rust have lifetimes. The term seems self-explanatory,
    but there are some enlightening details. Lifetime is the segment of "time"
    in which an entity exists. This is a consideration in all programming
    languages. We have previously discussed it when entities "fall out of
    scope." This common understanding is called "lexical lifetime." For
    example, once a block ends, that is the end of life for all of its
    contents.

    Rust's compiler is capable of sub-lexical lifetimes and allows entities to
    still be in scope but at the end of their lives. The compiler can tell if a
    reference is declared and then used before the end of a scope. Thus, the
    lifetime of a reference is actually from the point at which it is declared
    to when it is finally used. This is a key part of the borrow checker.
    
    This underlying behavior is not unique to Rust. What is unique is the
    concept of a "lifetime annotation." See below: */

    struct IBelieve<'a> {
        in_life: &'a str,
    }

    let when = "after love";

    let cher = IBelieve{
        in_life: when,
    };

    /* Whenever a value may be or will be a reference, such as in a struct or
    argument, it must have an explicit lifetime annotation. In the above
    struct, since a struct could be instantiated in a different lifetime block
    as its referenced values, the annotations specify that the struct instance
    has a lifetime of `a` and anything referenced in the struct must have _an
    equal or greater_ lifetime. */
        
    /* Lifetime annotations do not change an entity's lifetime. Instead, they
    are saving an entity's lifetime as an identifier. See the below: */

    fn some_function<'a>(x: &'a str, y: &'a str) -> &'a str {
        if rand::random::<bool>() {
            x
        } else {
            y
        }
    }

    /* In the above, lifetimes must be annotated because the compiler cannot
    infer the lifetimes of argument references passed in. The argument
    annotations declare a generic lifetime of 'a. Using single letters is
    simply convention, not a requirement. Next, `x` and `y` must have the _same
    as or greater than_ the base lifetime of the function. lifetime, and that
    the return value will have that lifetime as well. Essentially identical
    syntax is applied to implementation blocks.
    
    Lifetime annotations are not needed in many scenarios. The compiler will
    hold your hand. */


    /*** Static ***/

    /* Rust has one fixed and explicit lifetime: static. As you can guess from
    previous subjects, this lifetime applies to all items like the aptly named
    statics. The static lifetime means an entity exists for the entirety of the
    program's run. The only notable addition to this is that any string literal
    also has a static lifetime. This is because literals are part of the binary
    and are thus necessarily always in memory. */

    let static_string: &'static str = "Getting nothing but static on channel Z";

}

#[allow(unused_variables)]
#[allow(dead_code)]
fn pattern_matching() {

    /*----------------------------------------------
    * Pattern Matching
    *-----------------------------------------------
    */

    /* ALong with the usual forms of control flow, Rust includes the positively
    divine semantics of pattern matching. Pattern matching is the use of a
    "pattern" that is used to analyze something and determine the next step in
    the logical flow. The basic matching concept is similar to a regex, but
    full-featured pattern matching is much more powerful. To initially
    illustrate, let's use the Option from earlier. */

    fn generate_answer() -> Option<i32> {
        if rand::random::<bool>() {
            Some(42)
        } else {
            None
        }
    }

    let possible_answer = generate_answer();

    if possible_answer == Some(42) {
        println!("The answer is 42")
    }
    
    match possible_answer {
        Some(x) => println!("There is an answer and it is {x}!"),
        None => println!("There is no answer"),
    }

    /* Notice how the `Some()` is unpacked and its contents are given an
    identifier? Seems a bit heavy, no? The Rust team agrees. Options were
    common enough to earn inclusion in the language, and as such specialized
    syntax was also included for consuming them: the `if let`.
    
    `if let` allows for the `None` case to be silently ignored. If you delete
    the `None` part of the above `match`, you will get an error because the
    match cases are not exhaustive. You _must_ handle all possible cases. */

    if let Some(x) = possible_answer {
        println!("There is an answer and it is {x}!")
    }

    // The same syntax can be used to implement a while loop.
    let mut random = Some(rand::random::<bool>());
    while let Some(true) = random {
        println!("It's true!");
        random = Some(rand::random::<bool>());
    }

    /* Just as Option has syntax shorthand, so does Result. Instead of having
    to chain `match` expressions, a call that could return an `Err()` can
    simply have ? appended to it. In the below, both `result` and
    `another_result` can return errors. Chaining matches results in deeply
    nested pyramids almost like the old "Callback Hell" of JavaScript. With the
    `?`, if the error occurs, the function simply returns that error. Think of
    this like shorthand for a try/catch block. */

    enum ResultError {
        ErrOne(String),
        ErrTwo(String),
    }

    type TestResult = Result<i32, ResultError>;

    fn generate_result() -> TestResult {
        if rand::random::<bool>() {
            Ok(42)
        } else {
            Err(ResultError::ErrTwo(String::from("There was no answer")))
        }
    }

    fn check_result() -> TestResult {
        let result: i32 = generate_result()?;
        let another_result: i32 = generate_result()?;

        Ok(result + another_result)
    }

    let final_result = check_result();

    /* As mentioned earlier, optional arguments for functions require the use
    of Option(), which is best handled with pattern matching. Options _can_ be
    unsafely unwrapped, but since you shouldn't do that, I won't show you how.

    Again, if coming from a more free-wheeling language, this inability to
    elide arguments may seem overly restrictive, but it is those very
    restrictions that provide the extreme safety that is Rust's party piece.
    Learn it, live it, love it. */

    fn optional_args(op_arg: Option<i32>) {
        if let Some(val) = op_arg {
            println!("Found optional argument {val}")
        } else {
            println!("Found no argument")
        }
    }

    optional_args(None);
    optional_args(Some(42));

    /* The real power of pattern matching comes from more complex scenarios. */

    enum Magic {
        MagicMissile(i32),
        Fireball(i32),
        LightingBolt(i32),
    }

    fn generate_spell(spell: Magic) {
        match spell {
            Magic::MagicMissile(x) => println!("You cast magic missile and do {x} damage"),
            Magic::Fireball(x) => println!("You cast fireball and do {x} damage"),
            Magic::LightingBolt(x) => println!("You cast lighthing bolt and do {x} damage"),
        }
    }

    fn cast_spell() {
        let mut rng = rand::thread_rng();
        let spell = rng.gen_range(1..3);
        let power = rng.gen_range(0..99);
        match spell {
            1 => generate_spell(Magic::MagicMissile(power)),
            2 => generate_spell(Magic::Fireball(power)),
            3 => generate_spell(Magic::LightingBolt(power)),
            // This catch-all is required since the compiler does not know that
            // the value is within 1 and 4, it only knows i32. The choice of
            // `other` is arbitrary.
            other => println!("Invalid value of {other} submitted"),
        }
    }

    cast_spell();
}

#[allow(dead_code)]
fn opaque_types() {
    /*----------------------------------------------
    * Opaque Types
    *-----------------------------------------------
    */

    /* An opaque type is not really a type; it is a restriction. I like to call
    opaque types "specifics" to contrast them with generics. A generic means
    that a struct or function accepts one of all possible types, while a
    specific restricts the accepted type to a subset of all possible types.

    Opaque types can be bound to an identifier with the `type` keyword like so:
    
        type ThingWithToString = impl ToString;

    This is considered "unstable". This will be implemented at some point in
    the future. Moving on. */

    fn specific_function(x : impl ToString) -> impl ToString {
        print!("{}", x.to_string());
        "This is a string" // success
        // 42 // success
        // [42] // fail
    }

    /* In the above function, the parameter `x` is restricted to all types that
    implement the `ToString` trait. The function then returns any type that
    likewise implements ToString. The returned &str implements ToString, so
    this works, as does the integer below it. The array below that does not
    implement ToString and thus fails. Similar logic applies to arguments. 
    
    An important point to recognize is that while the return type is restricted
    to types that implement ToString, the function evaluation must collapse
    down to a single type. This type is called the "hidden" type, since it is
    not visible in the annotations. */

    fn two_hidden_types(x : bool) -> impl ToString {
        if x {
            return "This is a string"
        } else {
            // return 42 // Fails
            return "42"
        }
    }

    /* Even though the above function returns one of two things that both have
    the ToString trait, the hidden type of the function must be one or the
    other, &str or i32, it cannot be both.
    
    Unlike closures with un-typed parameters, calling a function with an
    opaque parameter type does not harden its hidden type. For example, the
    above specific_function() accepts an argument that implements ToString, so
    an integer would work. If specific_function() is called with an integer, it
    can still be called later with a &str.
    
    Functions like two_hidden_types() can be implemented, but this will be
    discussed later in the section on "dynamic dispatch." */


    /*** Monomorphization & Zero-Cost Abstractions ***/
    
    /* The reason for the difference between specific_function() and two_hidden_types() is a process that the Rust compiler does called "monomorphization." This is in contrast to the code that a developer writes, which is called "polymorphic." Polymorphic means "many forms" and describes specific_function() very well. The types that the function can accept take many forms, thus the function is flexible. But truly polymorphic code, as the default behavior of a function in JavaScript, suffers performance penalties, and Rust's goal is to be fast. Thus, the Rust compiler will take polymorphic code and transform it into "monomorphic" code. By that, I mean the compiler will create multiple versions of specific_function(), one for each use. For example: */

    specific_function(42);
    specific_function("a string");
    specific_function('Z');

    /* When compiled, a version of specific_function() that accepts an integer, a string, and a char will be generated. Thus a polymorphic function is turned into three monomorphic functions. This requires more memory but provides a significant speed benefit.

    The monomorphic transformation can easily generate functions that _accept_ different types, but because a function can be arbitrarily complex, it cannot generate functions that _return_ different types.

    This monomorphic transformation by the compiler is what is meant by Rust's motto of "zero-cost abstractions." They are abstractions that only exist in the code. To call them powerful syntactic sugar is accurate. */
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn smart_pointers() {
    /*----------------------------------------------
    * Smart Pointers
    *-----------------------------------------------
    */

    /* Pointers are obviously a memory address at which data can be found.
    Rust's most common pointer is the reference, denoted by the ampersand. But
    Rust also has "smart" pointers, which are semantic structures that contain
    a pointer along with some extra capabilities. */

    /*** Box ***/

    /* Box is the standard smart pointer and is extremely common in Rust
    codebases. It is a simple "box" that contains data of unknown size. The box
    can then be filled. If coming from C, think of it like the more symbolic
    version of `malloc`. */

    let boxed_int = Box::new(42);
    // Use of the box automatically, and safely, dereferences the pointer.
    println!("The box contains {boxed_int}");

    /* Because pointers are fixed size, they can be used in data structures
    with values of unknown size, like in a linked list. In the below, we cannot
    have a potentially infinite recursion of next nodes, so `next` simply
    points to another node. */

    struct LinkedList<T> {
        head: LinkedListNode<T>,
    }
    
    struct LinkedListNode<T> {
        value: T,
        next: Option<Box<LinkedListNode<T>>>, // If None, it's the end of the list.
    }

    /* When data exists in a box and not as a literal, programmers can attach
    their own logic to lower-level behaviors such as when the box is cleared
    from memory. Basically, it gives a developer visibility into lifecycle
    events. This is complex and outside the scope of this tutorial, but it is
    one of the major reasons for using smart pointers like Box. */

    
    /* Reference Counter */

    /* Because of Rust's concept of ownership being key to its benefit, Rust
    needs ways to enable _multiple_ ownership of an entity. For example, any
    graph where a node can be owned by multiple other nodes or edges. A node
    should only fall out of scope after _all_ owners are gone. Thus, when the
    count of references drops to zero, the node can be destroyed. A simple
    example is to just have two nodes pointing to one node.
    
    Look at the below struct. Only one node can have another node as its `next`
    value. Attempting to have two nodes point to the same node would cause an
    ownership error. */

    struct SimpleNode<T> {
        value: T,
        next: Option<Box<SimpleNode<T>>>,
    }

    /* Implementing the reference counter has some slight differences to Box.
    Reference counters must be instantiated, meaning that they cannot be boxed
    inside of the nodes. Each node must be an independent counter. Below, all
    three nodes are instantiated as counters, thus allowing an arbitrary number
    of other nodes to point to them. The call to Rc::clone does not actually
    clone the data, it simply increments the counter. I don't know why they
    chose that word. Note that the value of `next` requires a reference, as
    denoted by the ampersand. */

    struct SimpleRCNode<T> {
        value: T,
        next: Option<Rc<SimpleRCNode<T>>>,
    }

    let leaf_node = Rc::new(SimpleRCNode{
        value: 42,
        next: None,
    });

    let node_1 = Rc::new(SimpleRCNode{
        value: 2001,
        next: Some(Rc::clone(&leaf_node)),
    });

    let node_2 = Rc::new(SimpleRCNode{
        value: 420,
        next: Some(Rc::clone(&leaf_node)),
    });


    /*** Atomic Reference Counter ***/

    /* For allowing multiple owners across threads, Rust has the Atomic
    Reference Counter, Arc, as part of the standard library as well. See the
    section on concurrency for this description. */


    /*** Memory Leaks ***/

    /* Multiple ownership via reference counters allows for cyclical
    references, which will cause memory leaks. Basically, if A references B and
    B references A, neither of their counters will drop to zero. This is not
    prevented by Rust since there are scenarios where ciclical references are
    desirable. Chances are, you will never need this pattern, so while the Rust
    docs dedicate a significant amount of time to this scenario, and even
    specify a solution in the form of a "weak" reference, you should really
    just be aware of it so you can avoid it. */


    /*** Mutexes ***/

    /* A mutex is a smart pointer that only allows one entity to access it at a
    time. As with Arc, see the section on concurrency for a discussion of them.
    */
}

#[allow(unused_variables)]
fn modules_and_crates() {
    /*----------------------------------------------
    * Modules
    *-----------------------------------------------
    */

    /* Modules are similar in conception to modules in other languages. In
    comparison to other, object-oriented languages, a module also has passing
    similarities to classes. The primary purpose of modules is to hide types
    and/or functionality from other parts of the program.

    Every file in Rust is a module. Modules can have nested modules within them.
    Functions, too, can have modules in them. The visibility of modules, though,
    can be confusing if this pattern is used, as will be explained.
    
    The semantics of the below module should be familiar to most. Entities in a
    module are private by default, meaning they are not visible outside of it.
    Prepending the `pub` modifier makes that entity public and thus visible.*/

    pub mod stuff {

        struct PrivateStruct {
            x: i32,
            y: i32,
        }

        // Notice how the individual keys can be public or private.
        pub struct PublicStruct {
            pub x: i32,
            pub y: i32,
        }

        impl PublicStruct {
            pub fn multiply(&self) -> i32 {
                self.x * self.y
            }
        }

        // Only public structs can be exposed.
        pub fn get_thing() -> PublicStruct {
            PublicStruct {
                x: 42,
                y: 2001,
            } 
        }
    }

    /* Neither get_thing() nor PublicStruct are accessible outside of that
    module by simply calling their names, even though this module is a child of
    the main() function.

    The only thing that exists within the scope of main() is the module itself.
    As such, any usage of that module must call the module. */
    
    let new_thing = stuff::PublicStruct {
        x: 42,
        y: 2001,
    };

    /* Remember, classes are not a perfect analog for modules. Modules are
    static code organization and exist only at compile time. Modules contain no
    internal data. */

    let i_got_a_thing = stuff::get_thing(); // type of PrivateStruct

    /* As mentioned earlier, having modules sit next to one another can be
    confusing because it makes it seem like they can see one another. They
    cannot because they only exist next to each other in the code. See below. */

    mod more_stuff {
        pub struct MoreStuff {
            x: i32,
            y: i32,
        }
    }

    mod even_more_stuff {
        // impl more_stuff::MoreStuff {
        //     fn area(&self) {
        //         self.x * self.y
        //     }
        // }
    }

    /* `even_more_stuff` has no way to see `more_stuff`, even though it seems
    like it should. The complete inability of them to see each other is
    actually a side-effect of their being declared within a function.
    
    Similar modules have been declared outside of the function at the bottom of
    this file to illustrate how modules can interact. Go there now. */
}

#[allow(unused_variables)]
fn basic_operators() {
    /*----------------------------------------------
    * Operators
    *-----------------------------------------------
    */

    /* The reason for putting basic operators so late into this tutorial is
    because they are somewhat supercharged in Rust. Instead of relying on fixed
    operators, custom evaluators can be written, thus allowing engineers to
    decide how operators such as `>` or `==` function. Since this tutorial is
    not meant for beginners, I am not covering many of the absolute basics. */


    /*** Equality ***/

    /* In the below example, we want to use the equality operator on a struct.
    Rust comes with operator capabilities built in, but for custom structs these
    capabilities are not attached by default. The struct needs to be annotated
    with a #[derive] attribute that will generate the specified trait and attach
    it to the struct. Below, the PartialEq trait will be attached, thus allowing
    three equality operations. If the attribute is commented out, authors_1, 2,
    and 3 will fail to compile. */

    #[derive(PartialEq)]
    struct Author {
        name : String,
        age  : i32
    }

    let author1 = Author {
        name : String::from("Charles Dickens"),
        age  : 58
    };

    let author2 = Author {
        name : String::from("Charles Dickens"),
        age  : 58
    };

    let author3 = Author {
        name : String::from("Victor Hugo"),
        age  : 83
    };

    let authors_1 = author1 == author2; // - : bool = true
    let authors_2 = author1 == author3; // - : bool = false
    let authors_3 = author1 != author3; // - : bool = true

    println!("{} {} {}", authors_1, authors_2, authors_3);

    /* Arrays support ordinal operators by default, but the arrays must be of
    equal lengths, because remember, in Rust, the length of an array is
    actually part of its type. */

    let big_array = [10, 10000000, 10000000];
    let small_array = [11, 1, 1];

    let big_array = big_array == small_array; // - : bool = false

    println!("{}", big_array);

    let compare_authors_1 = author1 == author2; // - : bool = false
    let compare_authors_2 = author1 == author1; // - : bool = true


    /*** Custom/Overloaded Operators ***/

    /* Operators in Rust are actually just traits. As such, they can be
    "overloaded." For example, the equality comparisons above can be written
    thusly: */

    let compare_authors_3 = author1.eq(&author2);

    /* Note how the compared value must be a reference. Using an operator hides
    that detail.

    All other operators are likewise methods that can be overridden. */

    let add_method = 2.add(40);
    let sub_method = 44.sub(2);
    
    /* For a full list of operators that can be overloaded, see the Rust docs.
    What this means it that a developer can create a custom .eq() trait that
    will be called when the `==` operator is used.
    
    Be careful when overloading operators. Things can get confusing quickly.*/

}

#[allow(dead_code)]
#[allow(unused_variables)]
fn functions() {
    /*----------------------------------------------
    * Functions
    *----------------------------------------------
    */

    /* Rust is deeply inspired by functional languages, so unsurprisingly its
    functions are distinctly different from many other languages, such as those
    from the C family. This difference was briefly discussed earlier where the
    semantic use of semicolons was pointed out.
    
    To reiterate, Rust has en explicit `return` statement as a concession to the
    C tradition, but it also has implicit final return as denoted by the lack
    of a semicolon. In most cases, the final statement of a block being used as 
    its implicit return will be the ideal and idiomatic pattern.

    All evaluation blocks, and thus all functions, _must_ return something. If
    no final value is present, the block will return the special value `unit`,
    which will be discussed shortly. (There is a special case known as the
    `never` type that functions can also return, but this is esoteric and not
    useful to discuss or learn in this tutorial)

    For example, the below function has one evaluation block: the if/else. As
    such, this entire block is actually the return of the function. The if/else
    is composed of two evaluation blocks that each return a value. Thus, the
    two booleans count as the final return value of the function. */

    fn greater_than_42(x: i32) -> bool {
        if x <= 42 {
            false
        } else {
            true
        }
    }

    /* It is important to note that, even though Rust has a return statement,
    it applies only to the _function_ level, and not the level of general
    evaluation blocks. And only through the return statement can early return be
    achieved. To wit, while Rust allows a return statement, it restricts the
    semantics to avoid mixing up paradigms. Within evaluation blocks, only
    implicit returns are allowed, and Rust bars implicit early return.
    
    Let's break the below function to illustrate. */

    fn less_than_42(x: i32) -> bool {
        if x >= 42 {
            // false
            false
        } else {
            // return true;
            true
        }
        // if x < 42 {
        //     false
        // } else {
        //     true
        // }
    }

    /* if the first `false` is uncommented, it would lack a semicolon, and
    Rust's compiler would think that it is thus meant to be the block's return.
    But since there is a statement _after_ that, it knows that it cannot be the
    implicit return. It will thus throw a missing semicolon error.
    
    If the second if/else is uncommented, a similar problem arises. The second
    if/else becomes the implicit return of the function block, and thus the
    booleans contained therein become the return value of the entire function.

    The compiler knows that the implicit returns of the first if/else block are
    now not being caught by anything and will thus throw an error indicating
    that an explicit `return`, to thus break out of the entire function, was
    likely intended. This is why uncommenting the `return true` line does not
    throw an error, but instead throws a warning of unreachable code.
    
    This illustrates how the need for an explicit return likely means that the
    function has been poorly designed. Composing a function of evaluation blocks
    that all return values, and having the function itself finally evaluate to a
    final value, should be the ideal pattern. */


    /*** Anonymous Functions ***/

    /* Just like JavaScript and TypeScript, Rust functions can be "anonymous",
    meaning that the function itself has no identifier, but is instead bound to
    an identifier. The syntax is slightly different but likely very familiar to
    TypeScript developers who frequently use fat arrow function syntax. */

    let sign_up_to_newsletter = |email: &str| -> String {
        format!("{} {}", String::from("Thanks for signing up"), email)
    };

    /* In JavaScript, the above would look like this:
    
        let sign_up_to_newsletter = (email: string) : string => {
            return(`Thanks for signing up ${email}``);
        };

    */

    sign_up_to_newsletter("hello@rust_lovers.org");

    /* One of the most significant differences of Rust if coming from
    JavaScript/TypeScript or Go is that functions cannot access values declared
    outside of their scope. This is known as "capturing" a value. The common
    term is "enclosing," to wit you are writing a "closure", a concept I am
    sure many JavaScript developers remember from their job interviews. */

    let outer_var = 22;

    fn normal_function() -> i32 {
        let inner_var = 22;
        inner_var + 20 // This works.
        // outer_var + 20 // This does not.
    }

    /* The above is not possible because a `let` binding is part of the
    "dynamic" environment of the program. The dynamic environment is the part
    of the program that can change based on how the program runs. The "static"
    environment is the part of the program that is the same whenever the
    application runs. Because functions are static items, they do not exist on
    the same level as let declarations.
    
    If you are coming from TypeScript or JavaScript, you may interpret this as
    similar to hoisting, and while that is not entirely wrong, it is not
    entirely right. Functions do not get moved to the top of a scope, as they
    do in JavaScript. Functions, like all items, are lifted into a different
    realm. That said, the problems inherent to hoisting gives us a good
    illustration for why Rust works as it does.

        displayMessage();

        let message = "a message for you";

        function displayMessage() {
            console.log(message);
        }
    
    In the above JavaScript code, a function can be used before its
    declaration. But this code will fail because the `displayMessage` call is
    relying on `message`, which is declared _after_ the call. If Rust tried to
    allow the usage of functions with outside values, the function would not be
    able to know where to find this value. Thus, Rust simply prevents this.
    
    There are many uses for this pattern, though, and Rust allows it through
    the use of the aforementioned anonymous functions. Unlike JavaScript, where
    a function is only a closure if it encloses external values, Rust simply
    calls all anonymous functions "closures" as a way to differentiate them
    from normal functions. */

    let food = String::from("cookies");
    let closure_food = |x: i32| println!("You have {x} {food}");

    // Values captured by closures are borrowed by default.

    // let attempted_move = food; // This fails.
    println!("{food}"); // A simple reference use succeeds.
    
    closure_food(42);

    /* Just as earlier, mutable borrows are treated more strictly. Any closure
    which mutates its mutable captured values must also be labeled with the
    `mut` keyword and no references can be created between the declaration of
    the closure and its use. */

    let mut drink = String::from("coffee");
    let mut closure_drink = |x: i32| drink.push('s');

    // println!("{drink}"); // Simple references now fail.
    closure_drink(42);

    /* Borrowing is the default behavior but ownership can be transferred via
    the `move` keyword. The primary use of this is to transfer a closure, and
    everything it needs, to another thread. Multithreading will be discussed
    later. */

    let dessert = String::from("cheesecakes");
    let closure_dessert = move |x: i32| println!("You have {x} {dessert}");

    // println!("{dessert}"); // This fails.

    /* At this point, the value "cheesecakes" has not been dropped. It is
    instead bound to the identifier for the closure `closure_dessert`. Only
    once `closure_dessert` falls out of scope will the value be dropped. */

    /* Because closures are bound by let declarations, they are part of the
    dynamic environment along with the let values. As such, they can "see" each
    other.
    
    But Just as entities from the dynamic environment can enclose one another,
    functions can enclose other entities from the static environment. Both the
    below static value and constant value exist in the same realm as the
    function, so the function can indeed "enclose" them. */

    const OUTER_CONST: i32 = 42;
    static OUTER_STATIC: &str = "cookies";

    fn function_enclosure() -> String {
        format!("You have {OUTER_CONST} {OUTER_STATIC}")
    }

    /* Closures do not need type annotation. Since they exist within the
    lexical scope, the Rust compiler can infer types based on how the closure
    is used. This does not mean that the closure can be treated like a generic.
    The compiler will in fact harden the types after the first use. */

    let adder_closure = |x, y| {
        x + y
    };

    let answer_integer = adder_closure(20, 22);
    // let answer_float = adder_closure(2.0, 1.4159);

    /* If you uncomment the above, you will get a type error. This is because
    the usage of integers for `answer_integer` made the compiler infer the
    types of `adder_closure` to be integers. Thus, from that point forward,
    that is the type of `adder_closure`. This is true for all scopes in which
    `adder_closure` is visible. This is a hard restriction. Even if you pass
    `adder_closure` as a callback argument, the typing it acquires there will
    apply henceforth. */


    /*** A Note On Idiomatic Rust ***/

    /* The idiomatic use of closures in Rust is for small pieces of behavior
    that exist in small contexts. For example, a great many Rust libraries
    accept zero-parameter functions as arguments. These are usually written as
    inline, unbound closures. If coming from JavaScript, this will be
    exceedingly familiar with the .then() syntax.
    
    That said, the Rust compiler is intelligent. The ultimate difference
    between a closure with no captured values and a function is very small.
    While only using closures in restricted scenarios is considered idiomatic,
    if you want to use them in nearly every scenario, there is no real
    downside. */


    /*** First Class Functions & Dynamic Dispatch ***/

    /* Just as with most modern languages, Rust allows passing functions as
    values. Anonymous functions are truly first class and are passed like any
    other value, but regular functions can be passed as "function pointers,"
    which are preicely that: pointers to a function sitting in memory. */

    fn get_closure() -> Box<dyn Fn() -> i32> {
        Box::new(|| 42)
    }

    let a_closure = get_closure();
    let value_from_closure = a_closure(); // 42

    fn a_function() -> i32 {
        42
    }

    fn get_function() -> fn() -> i32 {
        a_function
    }

    let a_function = get_function();
    let value_from_function = a_function(); //42

    // Function pointers can also serve as simple aliases.
    let a_function_alias = a_function;
    let another_value_from_function = a_function_alias(); // 42

    /* Because function pointers are of constant size, they can be included on
    structs without any special considerations. */

    struct Strunction {
        func: fn(x: i32) -> i32,
        val: i32,
    }

    /* There are two things to note in the above: the usage of the `dyn`
    keyword and the capital F in Fn for the closure example.
    
    The type signature for get_function() makes sense. Functions are declared
    with fn, thus a function pointer is typed with fn. But the signature for
    get_closure() uses a capital F. This is because a closure is actually a
    trait. Closures are compiled into struct instances with a method attached
    to them that contains the actual logic of your closure. As such, the type
    signature of the closure is as a trait on that struct.
    
    Even though closures are traits, the `impl` keyword is not used because, as
    mentioned when discussing opaque types and monomorphization, when `impl` is
    used in a function signature, that signature represents an underlying
    concrete type. Closures have no underlying type. When pointing to a trait
    that exists on the heap, such as when having a Box<Trait> like in the above
    example, it is referred to as a "trait object." The `dyn` keyword was
    created to more clearly differentiate between implementations and trait
    objects. */

    /* Trait objects are Rust's way of handling what is known as "dynamic
    dispatch." If you are coming from JavaScript, or any scripting language,
    the concept of dispatch will be new to you. In compiled languages, there is
    a distinction between knowing what function will run, i.e. be dispatched,
    at compile time versus at runtime. For example, if an integer is greater
    than 0, function A will run, and function B will run if less than 0. The
    compiler does not necessarily know the value of the integer, but it doesn't
    need to. It knowns both roads perfectly, so it can walk either one equally
    quickly.
    
    But if the _function_ is not known, the compiler needs to find out what
    road it is to walk. If the function called is determined at compile time,
    it is called "static dispatch," meaning the behavior that is "dispatched"
    never changes. Dyanmic dispatch is the opposite of that. A synonymous
    description is "early binding" versus "late binding," where binding refers
    to the act of binding a value or behavior to an identifier. For example,
    `let x = 42;`. Rust's compiler knows that `x` is `42`, so it does not
    bother to check the value of `x` when running. This check is called
    "indirection." In JavaScript, every call to `x` theoretically requires the
    runtime to check `x` to see its value, although in practice runtimes will
    attempt to optimize this away.
    
    Dynamic dispatch provides significant flexibility in how a program runs but
    achieves it with a performance hit that can be similarly significant. In
    languages such as Python or JavaScript, the dispatch consideration is
    completely hidden. By and large, Rust's structure negates the need to
    consider dispatch. As mentioned, one of Rust's goals was "zero-cost
    abstractions," meaning that Rust features many very high-level language
    structures with great flexibility, but these "polymorphic" abstractions are
    made "monomorphic" at compile time. This means code can feel as though it
    is dynamically dispatching procedures while all functionality is actually
    static. */


    /*** Unit ***/

    /* You may have noticed in the above example of less_than_42, that if you
    uncommented the second if/else block, the specific error that was displayed
    was how "()" was expected, but a boolean was returned. In the previous
    section, I used the term "caught" when describing that the first if/else was
    returning something to nothing. That lack of a catcher for the evaluation's
    return means that Rust expected that block to return `unit`, or nothing. If
    there is no catcher, there should be nothing to catch.
   
    Unit is an interesting concept. It is the concept of a "thing" that is
    "nothing." It is different from `None` in that `None` is the state of
    nothing being where `Some()` could also have been. `Unit` is the state of
    expected nothing. It is similar to `void` in other languages, but unlike
    `void`, `unit` is actually a type, which is why the aforementioned unit
    struct is called a unit struct: it is actually an alias for `unit`.
        
    From a mathematical perspective, it could be seen as the empty set, in
    that it is still a set, but it is a set of nothing. */
        
    // Unit's first use is in declaring functions that take no arguments.
    fn no_argument() -> String {
        String::from("I've got nothing")
    }

    /* All functions necessarily return something, so if there is no expression
    intended for return, such as in functions that only handle side-effects,
    then that function will return `unit`. Functions that return `unit` can be
    explicitly typed. */
        
    fn no_return(input: String) -> () {
        println!("I just print {}", input)
        // "bingpot!" // This fails.
    }

    /* The above function expects to return nothing and will throw a compile
    error if anything is returned. */


    /*** panic! ***/

    /* While most errors will be handled with Results or Options, there are
    always scenarios where the failure should be terminal. For these situations,
    Rust has `panic!()`. panic is a macro that, when called, terminates the
    process in which it is called and "unwinds" its stack. Basically,
    everything in scope is destroyed and memory is freed. Since a panic exits
    the control flow of the program, the reason for the panic is likely unique,
    and thus the only information required by the compiler is a string. The key
    thing to remember is that if a function panics, the function that called
    the panic will also unwind. */

    fn maybe_panic() {
        println!("I'm looking for an answer");
        let what_im_looking_for = if rand::random::<bool>() {
            panic!("I panicked!")
        } else {
            42
        };
        println!("I found what I'm looking for. It's {what_im_looking_for}")
    }

    maybe_panic();

    /* If the above panics, the main thread is unwound and the rest of the
    program will not run.
    
    Panic should be a relatively rare tool, because most of the time you want
    to catch and handle errors. Panics should be used when your logic
    determines that the program has entered an entirely unexpected state. In
    essense, panics in Rust are what exceptions in other languages _should_ be:
    the machine state has fallen out of alignment with the symbolic state.
    Panics are used to fail tests. */

}

fn multithreading_and_concurrency() {
    /*----------------------------------------------
    * Multithreading/Concurrency
    *----------------------------------------------
    */

    /*** A Note On Concurrent vs Parallel ***/

    /* Concurrent and parallel are often used interchangeably, even in the Rust
    docs. Parallel is a subset of concurrent. Concurrent means that two
    processes are active simultaneously. Parallel means that the two processes
    are _also_ executing computations simultaneously. See the below
    visualization of processes A and B. Each "x" represents a unit of
    computation.
    
    A: x------x-----x-x----x---x---x------xxxx----x-x--x----x-|
    B: ----x-x----x--x---x---x-------xx--x-----xx-----x----x--|

    These are concurrent processes, but notice how the two lines never have
    moments of computational overlap. In parallel computing, there would, or at
    the very least could, be overlap. This usually means that there must be
    multiple computational units in the hardware. This could mean multiple
    cores, CPU-level multithreading, or specialized external processors such as
    audio chips, GPUs, NPUs, or in the olden days, math coprocessors.

    When writing Rust, all you can write are _concurrent_ processes. Whether
    they happen in parallel or not is out of your control. To a large degree,
    this is for the best. As a programmer, you cannot (easily) know how the
    hardware can most effectively run instructions simultaneously. For some
    interesting history on this, read about Intel's Itanium CPUs and their EPIC
    architecture. */


    /*** Fearless Concurrency ***/

    /* Rust was designed from the ground-up for concurrency. Many of its memory
    features were built with concurrent processes in mind. While concurrency is
    not as simple as something like Go, it is leagues simpler than either C or
    C++. Further, while Erlang, Elixir, or Go may be simpler, when done well,
    Rust's performance will be much better.
    
    To start, an important point is the nature of threads in Rust. Go and Java
    rely on "green" threads, which is a lightweight unit of concurrency that
    exists as a simple entity in memory that is controlled by the language.
    Because of this, Go can easily spawn tens of thousands of threads that the
    Go runtime juggles. Rust does not use green threads by default. It instead
    opts to use operating system threads. Spawning an OS thread is a
    significantly heavier and more complex operation than spawning a green
    thread but gives engineers more finely-grained control over how threads are
    created and managed.
    
    If you are coming from a higher-level languages like JavaScript, don't let
    this scare you. As I said, Rust is a great language because it gives
    programmers the _option_ to use lower-level functionality but provides
    libraries and tools that makes it surprisingly easy to use for the same
    goals as languages like JavaScript or Go.
    
    Just as all Rust applications have the main function, so too does that
    function represent the main thread. It is also the parent thread to any
    threads it spawns. Threads can spawn their own child threads. */

    /* All threads require a closure that encapsulates the desired behavior.
    Threads cannot borrow, though, so the below thread will fail to compile
    unless the `move` keyword is applied.*/
    
    let external_value = String::from("nee");

    thread::spawn(move || {
        println!("We are the knights who say {external_value}!");
    });

    /* From this point forward in the main thread, `external_value` is no
    longer valid. The value "nee" has _not_ necessarily been destroyed, though.
    Only once the child thread terminates would the value be destroyed in
    memory, and when the thread starts or terminates is impossible for the main
    thread to predict. The main thread may finish before the child thread can
    finish, thus destroying the child thread before it finishes. To prevent a
    parent from terminating before its children, the children can be "joined"
    to the parent. The `join` command becomes a part of the parent thread's
    lexical flow, meaning that the parent thread will stop until the child
    thread is complete before continuing. You can control when the parent
    thread pauses by choosing where to place the `join`. */

    let child_thread = thread::spawn(|| {
        println!("We are the knights who say Ekke Ekke Ekke Ekke Ptang Zoo Boing!");
    });

    // The main thread will pause here.
    child_thread.join();
    // The main thread will now continue.

    /* The above `join` command will trigger a warning about an unused
    "result". The result is the return of the child thread. The return is not a
    value per se, but a status. This is mostly about error handling. If a logic
    error happens in a thread, it "panics" and goes through a process called
    "unwinding" where its memory footprint is destroyed. When a thread is
    joined to its parent, the thread's status is monitored.
    
    The status returned is a boxed value that is either "ok" or an error. The
    box can be unwrapped, and thus the value is dropped:
    
        child_thread.join().unwrap();

    The value can also be bound to an identifier then simply ignored.
    
        let _ = child_thread.join();
    
    This tutorial's main() ignores unused variables, but if it didn't, any
    identifier other than `_` would trigger a warning. */


    /*** Channels ***/

    /* If coming from Go, welcome home. Channels are a fundamental part of Go
    and they are just as important in Rust. Channels are just a pipe into which
    one entity, a "producer," can put "messages" that are consumed by other
    entities called "consumer." This allows independent processes to
    communicate without having to share memory. Rust docs confuse terms by also
    calling them "transmitters" and "receivers." Even worse, Rust's typing
    information calls producers "senders." */

    /* We create a transmitter and receiver with the mpsc crate, which stands
    for "multiple producer, single consumer." The below example only uses one
    producer, but creating multiple producers is easily done by cloning. */

    let (transmitter, receiver) = mpsc::channel();

    // This creates a second producer. This must remain commented since a
    // dangling, unused transmitter will prevent a thread from completing.
    // let transmitter_2 = transmitter.clone();

    // The transmitter is then moved to a new thread.
    thread::spawn(move || {
        let important_people = vec!["Spongebob", "Prince", "Madonna", "Betty White"];

        for val in important_people {
            // Using a transmitter returns a Result. An error usually occurs
            // because the receiver has fallen out of scope.
            let result = transmitter.send(val);
            match result {
                Ok(_) => (), // Ignore success.
                Err(v) => println!("Error transmitting {v}"),
            }
        }
    });

    // For channels that may handle multiple messages, a for loop is used.
    // If only one value is returned, no loop is needed.
    for received in receiver {
        println!("{received} is an important person.")
    }


    /*** Mutexes ***/

    /* Mutex is a portmanteau of "mutual exclusion." If you are coming from C,
    C++, or Go, mutexes will be familiar. They are a common way to handle
    shared access to values. A mutex is mutually exclusive in that the value
    can only ever be accessed by one entity at a time. This prevents multiple
    threads from all trying to access a value simultaneously. Sharing state
    among threads has been a source of great difficulty, but Rust's rigid
    ownership rules makes using mutexes rather simple.
    
    The below example only uses this main thread. To use multithreading, the Arc
    smart pointer is needed, so the full example will come after. */

    // The syntax for mutex creation is identical to other smart pointers.
    let gigg = Mutex::new(23);

    /* Since a mutex needs to be mutually exclusive, any use must first lock
    it. This returns the boxed value which must be unwrapped to access. Of note,
    `gigg` is not technically the smart pointer. The lock method returns the
    smart pointer for use. */

    {
        let mut idy = gigg.lock().unwrap();
        *idy = *idy * 3;
    }

    /* At this point, the naked scope above is complete, `idy` falls out of
    scope and is destroyed, and the mutex is unlocked. */
    
    println!("{:?}, giggidy", gigg.lock().unwrap());

    /* The above uses string formatting syntax not previously discussed. If you
    are coming from C, C++, or Go, this syntax should be familiar. For values
    that do not implement the display trait, and can thus not be immediately
    included in strings, the :? unwraps that value. See more formatting
    ablities in the Rust docs: https://doc.rust-lang.org/std/fmt/index.html */


    /*** Arc ***/

    /* As mentioned earlier, sharing a value among multiple owners requires a
    reference counter. For sharing across multiple threads, the "Arc" type is
    required, for Atomic Reference Counter. They are atomic in the sense that
    they use "atomics". Atomics are a strange thing. Basically, a value is
    atomic if entities can only every view the value in a "complete" state. By
    that I mean that when values are changed, they can theoretically be in a
    intermediate state, and the nature of this intermediate state can be highly
    dependent on what the compiler does behind the scenes. Non-atomic values
    expose these intermediate states.
    
    To illustrate this, imagine an object with two integer fields: val and
    valx2. To update this object, the val needs to be updated then the valx2
    field needs to be computed. If the object can be viewed after val has been
    updated but before valx2 has been computed, the object is _not_ atomic. If
    the object can only every be viewed after a complete update has occured, it
    is atomic.
    
    But how does this thought experiment apply to real-world Rust? If three
    threads tried to use an Rc to, for example, increment a value of 39, they
    may all try to increment 39, making the final value 40 instead of 42. Arc
    prevents this by making changes seem instantaneous to all consumers of the
    value. Thread A would necessarily increment 39 to 40, thread B 40 to 41,
    and thread C 41 to 42. Atomic entities can be used independently as well
    with the atomic module in the standard library.
    
    The below example was mostly taken from the official Rust docs. I have
    added some comments and exploratory print lines. */

    let accumulator = Arc::new(Mutex::new(0));

    // A vector will store the thread "handles" for later join() calls.
    let mut handles = vec![];

    // Generate 10 threads.
    for i in 0..10 {
        let acc = Arc::clone(&accumulator);
        let handle = thread::spawn(move || {
            let mut num = acc.lock().unwrap();

            // The threads will likely print out of order.
            println!("Thread {} value is {:?}", i, num);
            *num += 1;
        });
        handles.push(handle);
    }

    // Iterate through the handles and join each one to the main thread.
    // Here I am using the unwrap() syntax discussed earlier.
    for handle in handles {
        handle.join().unwrap();
    }

    // The main thread will stop here and wait for the Arc to drop to 1, meaning
    // that all child threads have completed.
    println!("Result: {}", *accumulator.lock().unwrap());


    /*** Deadlocks ***/

    /* Just as Rc has the danger of circular references, thus resulting in a
    memory leak, Mutex has the danger of "deadlocks." In a deadlock scenario,
    thread A needs x and y, and thread B _also_ needs x and y. If thread A
    locks x and thread B locks y, then both threads will sit there waiting for
    the other value that they need to be unlocked. Thus, neither thread will
    ever finish. There are best practices for avoiding deadlocks that are
    outside the scope of this tutorial. */
}

#[allow(unused_variables)]
async fn async_syntax() {
    /*----------------------------------------------
    * Async
    *----------------------------------------------
    */

    /* Asynchronous Rust, henceforth called async, is a comparatively new
    addition to Rust semantics. It is actually still technically in flux, with
    breaking changes being implemented, but it has been broadly stable for a
    couple of years. That said, _in my opinion_, unless you are using a library
    that relies on async such as Actix-Web, you should prefer using traditional
    threads. Hopefully, async will fully stabilize in the near future.
    
    As opposed to default concurrent Rust, async Rust uses what can be
    described as green threads. Async is perhaps a new concept to those coming
    from Go, C, C++, or Java, but for JavaScript developers, welcome home.
    Everything covered here will be very familiar. There are implementation
    details, but those should arguably be hidden. You can read about them in
    the partially-completed async documentation.

    Async functions, when called, do no work. Instead, they return a "future".
    This is synonymous to a "promise" in JavaScript. Unlike promises, which
    immediately return a boxed promise _and_ begin running the function,
    futures return the box but do not run the function. The function must be
    "polled". Polling is done with the `await` keyword. If you are coming from
    Python, a language to which I have paid little attention, this pattern
    should be familiar. This means that Rust more strictly enforces what can
    call an async function. Unlike JavaScript, where any function can call an
    async function, in Rust, _only_ async functions can call other async
    functions.
    
    The second key difference is that async operations in Rust are not part of
    the language per se, but instead a standard syntax around multiple possible
    implementations from which you can choose. The most common async
    implementation is Tokio, but there are others with different strengths.
    When using Tokio, the library creates a thread pool with which it handles
    your asynchronous behaviors. Basically, you are handing over thread
    management to a library and you should consider your use of async as you
    using a library and not "real" Rust.
    
    The third key difference is that, because most everything in Rust is an
    evaluation, blocks can also be labeled as async. */


    /*** Initializing the runtime ***/

    /* This will be the strangest part to developers from other languages like
    JavaScript. You must start your async runtime before using async.
    
    Most of the time, if you are using async, it will be a key part of your
    application. As such, your main() function will be labeled as async. It
    requires the #[tokio::main] attribute, otherwise the compiler will throw an
    error. For this tutorial, I have labeled the main() function. Other
    runtimes may have other methods of initialization. */


    /*** Functions ***/

    // Just like JavaScript, `async` indicates an async function.
    async fn async_function() -> String {
        // Do something asynchronously like maybe get some data.
        String::from("Here's some data")
    }

    /* Notice how the await is not a method. This is because a method implies a
    function call, while the await is not exactly that. It is a keyword and is
    semantically similar to the `await` being before the function call as in
    JavaScript. Under the covers, it transforms the code. The `.await` you see
    is syntactic sugar */
    
    let some_data = async_function().await;
    println!("{some_data}");


    /*** Closures ***/

    // The below is technically unstable.
    // let async_closure = async || println!("Got data!");
    
    /* The below is the accepted current solution but is fundamentally
    different to the above. In the above, the function is not run and thus no
    stack space is allocated. In the below, the function _does_ run, but it
    immediately returns a block wrapped with a future. The performance
    difference is likely tiny, but worth noting. */
    
    let async_closure = || async { String::from("More data!") };
    

    /*** Blocks ***/

    /* Because nearly everything in Rust is an evaluation, that means that
    entire code blocks can be tagged as async. Since async blocks necessarily
    return a future, naked scopes/blocks cannot be labeled as async. */

    let async_block = async {
        let some_data = String::from("Data from a block");
        println!("{some_data}")
    };


    /*** Streams ***/

    /* Async in Rust, being fundamentally a library, unsurprisingly includes
    some features found in other language's libraries. The feature that stands
    out to me are streams. A stream is a future that can return multiple values
    at unknown intervals. A stream can live for an arbitrary length of time.
    The below examples use the Futures library and a simple async stream
    implementation developed by the Tokio team. Other libraries and rutimes
    will have broadly similar syntax. Especially if coming from JavaScript, all
    of this will be familiar. */

    let cross_the = stream!{
        let v = vec![42, 2001, 314, 1999];

        for val in v {
            yield val;
        }
    };

    /* This macro is an easy way to "pin" a value. A pinned value means that it
    will remain in the same memory location for its entire lifetime or until it
    is "unpinned". Since async code runs at indeterminate intervals, ensuring
    it is reliably positioned at all times is necessary. */
    
    pin_mut!(cross_the);

    // Async values require the use of while loops. For loops are in progress.
    while let Some(value) = cross_the.next().await {
        println!("{value} is an important number");
    }
}

#[allow(unused_variables)]
#[doc(hidden)]
fn macros() {
    /*----------------------------------------------
    * Macros
    *----------------------------------------------
    */

    /* Macros are one of Rust's superpowers. It is almost funny to say that
    considering that macros go all the way back to the dawn of high-level
    programming, but they are a capability that most programming languages have
    ignored. There are fundamental reasons for this that are outside the scope
    of this tutorial, but suffice it to say that it is because macros in the
    sense I am using the term require a rigidly symbolic language to implement,
    and most languages are not rigidly symbolic. They have flex and give in
    their syntax and semantics.
    
    Macros, in essence, instruct the compiler to write code for you. At compile
    time, a macro takes provided "tokens," often just a string, and turns those
    tokens into full Rust code. This is what I meant when I said that macros
    "expand." This expanded code then runs like any other piece of code that
    was written by the programmer themself.
    
    At first, this sounds like a roundabout way to write a function. Why
    generate code at compile time? Why not just write it? Basically, this is a
    layer of abstraction. A programmer is disconnecting their _desires_ from
    the _implementation_. It is declarative programming at a very low level.
    Indeed, when using the generic term "macro," like the macros used in Lisp,
    we are actually discussing what Rust explicitly classifies as "declarative
    macros." These are the macros previously discussed and used, like println!
    and vec!. Fundamentally, declarative macros analyse the pattern of provided
    tokens and generate code based on the pattern. The generated code then
    replaces the macro call site. That sounds very similar to a compiler
    because it _is_ very similar to a compiler.

    Rust has a second type of macro called "procedural" macros. They are so
    called because they are semantically similar to "procedures," which are,
    for our purposes, synonymous with functions. They take code as input and
    return code as output. In Rust parlance, this code is called a token
    stream. The distinction between declarative and procedural macros may seem
    small. While declarative macros can accept any syntax, procedural macros
    only accept valid Rust code as an input. And while declarative macros
    actually replace the macro call with the generated code, procedurals do not
    replace the code, but instead alter or augment it.
    
    For example, the previously used #[derive(PartialEq)]; this generates an
    implementation of a trait for the struct that directly follows the macro
    call.

    Procedural macros must be in their own crate, meaning I cannot put examples
    in this file. See the official docs and the Little Book Of Rust Macros for
    more information.
    
    Rust macros are known as "hygienic" macros, meaning they cannot access
    values outside of their context and will not accidentally generate code
    that interferes with external entities.
    
    With all that said, understand that most developers will not write many of
    their own macros. They are best when made as generic as possible, which
    means most macros that you would want exist in libraries. */


    /*** Declarative Macros ***/

    /* If declarative macros behave like functions, why not just use functions?
    Because Rust is strict in its code shape, macros enable flexibility by not
    being code themselves. Instead, they generate strict code. For example, it
    is impossible to write a function that accepts an unknown argument shape.
    This is exactly what println! does. It can accept a single string or a
    string and any number of ancillary arguments.
    
    The downside of declarative macros is primarily just complexity. The
    special syntax required to automatically generate Rust is dense and becomes
    unwieldy when generating large pieces of code. The official Rust docs have
    a simplified implementation of the vec! macro that I will copy here. I have
    renamed it to simple_vec. */

    macro_rules! simple_vec {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }

    let a_simple_vector = simple_vec!([1, 2, 3, 42]);

    /* A full exploration of the special syntax, called macro_rules! syntax, is
    outside the scope of this tutorial. Read the Rust docs for more
    information. But even eliding the details does not prevent the matching
    syntax from being pretty clear. The first line accepts $x as an expression,
    the * matches an arbitrary number of expressions, and it then generates a
    push for however many expressions there were. It feels a bit like a
    templating language because, really, it is.
    
    The ultimate effect of a declarative macro is that the macro call is
    replaced with the code specified in the template. */

}

/// This is a Rustdoc outer document comment.
/// 
/// It can stretch over multiple lines.
/// Most IDEs will allow you to collapse it.
#[doc(alias("documentation", "bingpot"))]
fn rustdoc() {
    /*----------------------------------------------
    * Rustdoc
    *----------------------------------------------
    */

    /* Rustdoc is similar in intent to JSDoc for those coming from JavaScript.
    Unlike JS, though, the tool is included in the standard Rust distribution.
    Rustdoc will take the documentation blocks at the top of functions and
    other objects and generate a web page that allows people to explore the API.
    
    It is important to note that Rustdoc does not rely on Cargo. Instead, it
    relies on the same things on which Cargo also relies, notably the Rust
    compiler. This means that correctly configuring your Cargo.toml file does
    not necessarily mean that Rustdoc will work or that it will work as
    expected. In most cases, this simply means that your Rustdoc command will
    make use of numerous flags. But of course, you can also just run Rustdoc
    _through_ Cargo, which will correctly handle all of the various flags. This
    is how most engineers will generate docs. Simply run this command:

        cargo doc

    Navigate to /target\doc\rust_quick_guide and open the index.html file in a
    browser.
    
    Just as with attributes, Rust differentiates between inner and outer doc
    comments. Outer comments are similar to systems like JSDoc, where the
    comment directly precedes the entity it documents. These outer comments are
    preceded by _three_ forward slashes. You can see one of these comments
    directly above the rustdoc() function.

    A useful tool for organizing large libraries and applications are the
    documentation aliases seen above this function. It allows for arbitrary
    strings to be attached to a function for the purpose of searching.
    
    Finally, often, functionality that is not meant for public consumption may
    have documentation automatically generated. To prevent this, use the
    #[doc(hidden)] attrbitue. An example of this can be found above the macro()
    function just above the rustdoc() function. When docs are generated,
    macros() will not be included in the output. */

}

fn actix_web() {
    /*----------------------------------------------
    * Actix-Web
    *----------------------------------------------
    */

    /* It may seem initially strange to include an external library as part of
    a tutorial, but I want to capture the people who may be coming here from
    Go, Java, or Node and are interested in Rust primarily as a tool for
    developing n-tier applications.
    
    A common refrain from programmers online is that Rust and Go are different
    languages and shouldn't be considered in opposition. I disagree with this.
    Rust does indeed have a higher learning curve than Go, but once learned its
    semantics make perfect sense. Further, Rust's libraries mean that it can
    easily be cantilevered into most use cases while maintaining development
    speed.
    
    The ne plus ultra exemplar of this can be found in Actix-Web. Rust was
    already well known, but the emergence of Actix-Web was one of the most
    salient moments driving Rust's fame and recognition. When a new framework
    comes out of nowhere and promptly tops every benchmark, people take notice.
    This early fame means that Actix has become the default framework. I am
    including this section here to show how easy n-tier development can be and
    how familiar it can feel. Also of note, Actix relies on Tokio for its async
    runtime and the previous section on async nicely connects here. */
}

/*----------------------------------------------
* Testing
*----------------------------------------------
*/

/* This section is dedicated to testing since tests cannot be nested. They must
be direct descendents of a module. They are wrapped in their own module here
because how the test attribute is interpreted is based on what imports are
found in the module. The conflict for this tutorial comes from the Tokio
import. 

As briefly mentioned, testing in Rust is done with simple attributes atop
functions. Including this syntax with the language spec itself has become
common in newer languages and enables the popular pattern of colocating
implementation code and testing code to be idiomatic. I appreciate the
JavaScript world's separation of testing and language, which has resulted in
significant innovation in its build and test ecosystem, but I ultimately prefer
standards.

The test-writing experience in Rust is very similar to patterns in other
languages, so there shouldn't be many surprises. There are two notable
differences from other languages. Firstly, unlike languages such as JavaScript,
Rust does not allow testing for function calls. If you want to check for a
function call, you must leverage dependency injection. Secondly, Rust does not
enforce privacy for functions, meaning that tests can access public or private
behavior.

The below functions will run with the `cargo test` command. Rust uses multiple
threads by default, so make sure to avoid using shared state. If you want
predictable control of shared state, use the `--test-threads=1` flag to only
use a single thread.

Rust does not display any output from the code, such as from println!, by
default. You can enable it with the `--show-output` flag.

Tests can be ignored by default with the #[ignore] attribute. This is commonly
used on long-duration tests to prevent a full test run from taking forever. The
ignored tests can be run with the `--ignored` flag. ALl tests, regardless of
status, can be run with `--include-ignored`.

Specific tests can be run using the same double-colon path syntax used to
reference modules.

    cargo test testing_stuff::test_panic 

The above will only run one test. To run multiple tests, passing in any
incompletely identifier will run all tests that match the provided string.

    cargo test testing_stuff

The above will run all tests in the testing_stuff module.

    cargo test multiplier -- --include-ignored

The above will match "multiplier" to test names. This means that both tests
with the word "multiplier" in their identifier will run. */

#[allow(dead_code)]
fn mult_by_two(x: i32) -> i32 {
    println!("Multiplying");
    x * 2
}

#[cfg(test)]
mod testing_stuff {
    use crate::mult_by_two;

    #[test]
    fn test_multiplier() {
        let result = mult_by_two(21);
        println!("The test result is {result}");
        assert_eq!(result, 42); // Change this to see a failure.
        assert_ne!(result, 41);
    }

    #[test]
    #[ignore]
    fn test_multiplier_2() {
        let result = mult_by_two(21);
        assert_eq!(result, 42); // Change this to see a failure.
    }

    #[test]
    #[should_panic(expected = "ran away")]
    fn test_panic() {
        panic!("I got scared and ran away");
    }
}


/*** Integration Tests ***/

/* As mentioned, Rust allows unit testing of private functions. Rust instead
uses privacy restriction as the primary differentiator between unit tests and
integration tests. Integration tests are again similar to how they exist in
other languages. They consume your code as though they were an external entity.
Integration tests turn each test file into its own crate which imports your
code, meaning that integration tests are best used when writing a library.

Appropriately, the tests sit inside the /tests directory that is a sibling of
the /src directory. Since the test files must exist in this location for the
compiler to correctly manage them, they are outside the scope of this one-page
tutorial. For full details see the Rust docs. */


/*----------------------------------------------
* Modules Part Deux
*----------------------------------------------
*/

/* This content is part of a section in the above function. Do not read it
separately.

These modules are not nested inside of a function. They are in the base scope of
the file and thus exist on the module level. They can thus see each other. */

#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
mod external_stuff {
    pub struct Stuff {
        pub x: i32,
        pub y: i32,
    }
}

mod more_external_stuff {
    // A use path that references the crate can be used, but since the modules
    // are siblings, this is not necessary.
    use crate::external_stuff::Stuff;

    pub fn get_stuff() -> Stuff {
        Stuff {
            x: 42,
            y: 2001,
        }
    }

    // super:: can also be used. It references the _parent_ module, unlike the
    // use crate:: above, which references the root of the project.
    pub fn get_stuff_2() -> super::external_stuff::Stuff {
        super::external_stuff::Stuff {
            x: 42,
            y: 2001,
        }
    }

    mod nested_module {
        pub struct NestedStuff {
            pub x: i32,
            pub y: i32,
        }
    }

    // To access sibling sub-modules, the self:: selector is used.
    pub fn get_nested() -> self::nested_module::NestedStuff {
        self::nested_module::NestedStuff {
            x: 42,
            y: 2001,
        }
    }
}
```