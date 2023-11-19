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
short supply. As such, the excitement and successes of the real world were
defined by the first "hackers," who were people less concerned with what symbols
could do and more concerned with the cool things that could be achieved with
direct control over the machine. This divide between the symbolic purists and
the hackers would continue into every successive generation of geeks. The
narrative thread connecting Rust and C even goes back to the very beginning of
computing. The first people who could reasonably be called programmers, Ada
Lovelace and Charles Babbage, separated along the same lines. Lovelace was a
self-described metaphysician who recognized the philsophical possibilities in
Babbage's machine, while Babbage himself cared about the machine itself and its
ability to do mathematics.

I see another thread running through modern computing that appears to be turning
back on itself. While the hackers were the ones driving things, the eras of
computing have been defined by the symbolics of the age. The earliest computing
was defined by languages intended to make computing more approachable for all
and sundry. COBOL, Fortran, and especially Basic were intended to let people
interact with computers in less arcane ways. They served a purely human purpose.

These languages powered the usage of computers. That is to say that COBOL was
used by companies that used computers to write solutions to business problems.
They were not used by the companies making software to sell or by companies
making the computers themselves. For them, theirs was a world of assembly
programming--high performance and directly tied to the machine.

This era continued until the rise time sharing.

C was the first language that attempted to give the geeks a new language. C
sacrificed some speed to make it easier to write for more machines. It was
called "portable aseembly."

Java took this one step further by eliminating the compile step and running
programs in a virtual machine.

JavaScript brought this process to its ultimate conclusion, which is why it is
everywhere.

But now we are reverting. Performance is becoming important again.

There are moral reasons. A bad program can produce much more CO2 than a good
program. But I think the bigger motivation is because cloud computing is
extremely expensive. If an organization is genuinely only "paying for what you
use," then a language that is twice as fast as another could result in a massive
decrease in monthly cloud charges.

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
use core::num;
use std::array;

const _GREETING: &str = "Stay awhile. Stay foever.";

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
the memory is cleared.

The heap is precisely that: a big pile of memory space. The primary
differentiator between stack and heap entities is that anything on the stack
must be of known and fixed size. Anything that can change in size must exist on
the heap. */

/*** The Main Function ***/

/* The below is the main function which all Rust applications have. This quick
guide does not cover the structure of Rust applications. It focuses on the
syntax and semantics. For the purposes of this guide, it is sufficient to know
that Rust applications have a main function.

The syntax structures above the function are called attributes. They allow a
developer to specify how the function is the be handled by the compiler. In
these examples, three linting settings are being disabled. More attributes will
be covered later. */

#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
#[allow(unused_mut)]
fn main() {

    /*----------------------------------------------
    * Variables, functions, and bindings
    *-----------------------------------------------
    */

    /* In the tradition of the ML language family that is Rust's inspiration,
    variables are not, by default, variable; they are bindings. As such, even
    though the term "variable" is commonly used, the more accurate term is
    "identifier." Thus, values are bound to identifiers, and only through
    identifiers can values be accessed (usually).
    
    Unlike the more pure, academic languages of OCaml and Haskell, Rust makes
    concessions to real-world performance. Making values mutable increases
    performance. Rust's compromise is to make values immutable by default. */

    let immutable_value = 42;

    /* Note that the above binding is an evaluation, meaning the right-hand side
    is an expression. Expressions are code that return a value. The majority of
    Rust code statements are expressions. The above is identical to: */

    let immutable_value = { 42 };

    /* This also illustrates a concept in Rust called variable shadowing. This
    is far from unique to Rust but allows an engineer to re-declare a
    variable and have the new value apply henceforth. Shadowing allows the
    type to be changed because, even though the name is the same, it is actually
    a _different_ identifier, which is why we can re-declare the value as
    mutable. */

    let mut immutable_value = 2001;

    // A mutable value is created with the `mut` modifier.
    // Mutable values must still abide by the type of the original binding.
    let mut mutable_value = 42;
    mutable_value = 2001;

    /* At this point, it is important to discuss the relationship of values and
    identifiers. Both values and identifiers are entities existing in memory. It
    is the identifier that determines how the program will interact with the
    value. Values are not inherently mutable or immutable -- again, they are
    just 1's and 0's in memory somewhere -- it is whether the `mut` modifier is
    present that determines if the program will _treat_ that value as mutable or
    immutable.
    
    This may seem like an esoteric restatement of what was already said, but it
    is important to understand this relationship to better grasp the later
    discussion of Rust's party piece, ownership. */

    /* TODO: Talk about how variable names can reference earlier versions of the
    same name and how this pertains to ownership. */

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
    run to determine. */

    const THE_ANSWER: i32 = 42;
    const HALF_THE_ANSWER: i32 = THE_ANSWER / 2;
    // const RANDOM_NUMBER: i32 = generate_random_int(); // Fails.

    /* These behaviors make sense. Constants exist to provide a high-performance
    method for storing data of known, fixed size for sharing across parts of the
    application. The global state should not be dynamic. */

    /*** Type Inference ***/

    // Rust can usually infer types.

    let x = 42;                // 32-bit unsigned integer.
    let y = 3.14;              // 64-bit float.
    let z = "Dinner for one"; // &str, a primitive string of fixed size.

    // Functions cannot infer parameter and return types.
    fn add_ints(a: i32, b: i32) -> i32 { a + b }

    /*** Scope ***/

    /* Scope is the "space" in which entities exist and can thus be accessed.
    Scope is especially important in Rust because it is how Rust's memory
    management works. */

    // Binding scopes are delineated by curly braces.
    if true {
        let val1 = "best of times";
        println!("{}", val1)
    };

    let my_block = {
        let val1 = "worst of times";
        println!("{}", val1)
    };

    // Blocks are expressions, meaning that the last statement in the block is
    // implicitly returned. Notice how a semicolon defines the termination of an
    // evaluation block, meaning that no semicolon indicates the final return.
    let my_block_value = { // Is set to 42.
        let x = 20;
        let y = 22;
        x + y
    };

    /* The list of reserved words can be found here:
    https://doc.rust-lang.org/reference/keywords.html

    To see the unused variable warnings below, comment out
    #[allow(unused_variables)] near the top of this file.

    Prefixing a variable name with an underscore creates a casual variable.
    These variables, if unused, will not trigger a compiler warning.
    
    Unused variable warnings only apply to block scopes and will not be
    raised on variables declared at the global or module level. 
    
    There are two kinds of unused variable. A suspicious unused variable is one
    that has been bound with `let`. An innocuous variable is one that has not.*/

    let block_scope = {
        let variable = 42;           // This triggers a warning.
        let _casual_variable = 2001; // This does not trigger a warning.
        "Return string"
    };

    /*** References ***/

    /* As a memory managed language, how data is passed around is a key part of
    managing that memory. For those coming from TypeScript, this will seem like
    a new concept, but it's not _that_ new.
    
    A reference is simply a pointer to something. If coming from TypeScript, you
    are aware of the danger with passing an object or array to a function, since
    if you mutate that object or array inside of that function, it gets changed
    _outside_ of the function. That is because what you passed into the function
    was a reference to the object or array, not the object or array itself.
    
    The only major difference in Rust is an engineer can explicitly reference
    _any_ value, not just objects or arrays, by prepending an ampersand to the
    variable name. */

    let a_basic_number = 42; // type of i32
    let another_number = a_basic_number; // another i32
    let a_basic_reference = &a_basic_number; // type of &i32

    /* In the above, `a_basic_number` is a 32-bit integer. When referencing
    primitives, they are copied by default, so `another_number` is also a 32-bit
    integer. But if an ampersand is prepended to the variable usage, it becomes
    a reference that holds no value, but instead points to the location of
    another value. The copy does not occur.
    
    There is more to be said about references, since Rust's management of values
    is its party trick, but we will get to that in the section on ownership. */

    // TODO: Cover that. Copy the original values entirely to go back over what
    // was previously discussed.

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
    developer will never do. "Safe" Rust's performance is already so good that
    unsafe Rust is unnecessary in all but the most demanding situations. Unsafe
    Rust will be explored fully later.
    
    Because pointers in Rust are deeply connected with Rust's patterns of
    memory management, a full explanation this early into the tutorial is not
    ideal. As such, a full explanation of pointers will happen later. */

    /*----------------------------------------------
    * Primitive Types
    *-----------------------------------------------
    */

    /*** > Integer ***/

    let val1 = 1 + 1;          // i32 = 2
    let val2 = 25 - 11;        // i32 = 11
    let val3 = 5 * 2 * 3;      // i32 = 30
    let val4 = 8 / 2;          // i32 = 4

    // Integer division will round results
    let val5 = 8 / 3;           // i32 = 2
    let val6 = 8 / 5;           // i32 = 1

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

    /*** > Float ***/

    let float1 = 1.1 + 1.5;     // float = 2.6
    let float2 = 18.0 - 24.5;   // float = -6.5
    let float3 = 2.5 * 2.0;     // float = 5.0
    let float4 = 16.0 / 4.0;    // float = 4.0

    // Floats and integers can be formatted with underscores. These underscores
    // are purely visual.
    let formatted_int = 12_34_56;       // i32 = 123456
    let formatted_float = 12_34_56.0;   // f64 = 123456.0

    /*** > Char ***/

    /* Char represents Rust's true text primitive. Strings are best thought of
    as an array of chars. Rust supports unicode and as such a char is actually
    a scalar value. In code, a char can be represented either as a glyph or as
    the scalar values as found on the Wikipedia page for Unicode characters:
    https://en.wikipedia.org/wiki/List_of_Unicode_characters 
    
    Be careful, though, because treating a glyph like a char is not always
    correct. There are many singular glyphs that actually require multiple chars
    to represent. This is primarily a concern when converting from a string. */

    // Use single quotes for a char.
    let last_letter = 'z';
    let last_letter_unicode = '\u{007a}';
    let kannada_char = 'ಠ';
    let kannada_char_unicode = '\u{0ca0}';

    /*** > Array ***/

    /* If you are coming from C, Java, or Go, then arrays in Rust will be
    immediately familiar. If you are coming from TypeScript or JavaScript, they
    will be a little different. Arrays in Rust are typed, fixed-length lists of
    entities. Unlike TypeScript, you cannot simply push to an array because an
    array's length must be set when it is instantiated. JavaScript runtimes hide
    this complexity from you when using arrays. */

    // The length of an array is part of the identifier.
    let array_of_five: [i32; 5] = [1, 2, 3, 42, 5];

    // Arrays cannot be instantied with no values. Unless the length is
    // specified as zero. An element type is still required.
    let array_of_none: [i32; 0] = [];

    // Arrays can be instantiated with the same value in all indices, though.
    let array_of_ten: [i32; 10] = [42; 10];

    /* The value syntax above is simply saying to create an array with the value
    of 42 set in 10 indices. */

    // Arrays are index accessed.
    let the_number_3 = array_of_five[2];

    // Array lengths can obviously not be changed, but neither can values.
    // Arrays must be declared as mutable to do this.
    let mut mutable_array_of_five: [i32; 5] = [1, 2, 3, 42, 5];
    mutable_array_of_five[2] = 314;

    println!("{}", the_number_3);
    
    // TODO: Talk about arrays being on the stack instead of the heap.
    // TODO: Talk about how accessing an index does not take ownership. Only the whole array access.

    /*** > Slice ***/

    /* Slices are the primary tool with which you will interact with arrays.
    Slices are very similar to slices in C++ but notably different than in Go.
    In Rust, a slice is simply a window into a preexisting sequence of data. For
    the sake of this section, that sequence of data is an array.

    Go's slices behave similarly to arrays in JavaScript or TypeScript, to wit
    a slice can be instantiated independently of an array. The Go runtime will
    instantiate an array behind the scenes that will grow or shrink based on the
    slice. This makes the developer experience nicer but has performance
    implications. */

    // Referencing an array automatically creates a slice.
    let slice_of_five = &array_of_five;

    /* Slices are more generic in Rust than other languages, and as such they
    can be used with more than just Arrays. Most importantly, slices can be
    used with strings, which will be discussed now. */

    /*** > Vector ***/

    /* For Go developers accustomed to slices and JavaScript/TypeScript
    developers accustomed to arrays, Rust has a "vector." Vector is not a true
    primitive and is instead part of the standard library. Vectors are typed
    lists, similar to arrays, but are dynamically sized and exist in the heap. */

    /*** > String ***/

    /* Strings in Rust are not true primitives in the sense that a primitive is
    a thing of known, fixed size. They are like C strings in that they are best
    described as an array of characters, with each character being the true
    primitive.
    
    The fundamental type, to wit the type that is part of the language itself,
    is `str`, which is called a string slice. The `String` type is actually part
    of the standard library and is best described as a wrapper around `str` that
    provides helpful functionality. Because of the common usage of `String`, the
    two types are often confused in conversation, with people using the term
    "string" to refer to either `String` or `str`. This tutorial will be
    explicit, unly using the word "string" to refer to `String` types.

    But what will the `str` be called? Why "string slice" of course! This
    terminology requires an understanding of slices, which will be explored
    fully later. For people coming from languages such as C++ or Go, slices in
    Rust are extremely similar. For the time being, just understand that a slice
    is a window into a contiguous sequence of entities in memory.
    
    For example, "ABCDEFG" are chars sitting next to each other in memory. A
    slice can represent all of them, some of them, or none of them. Rust does
    not have a type for representing the sequence of chars itself. */

    // Use double quotes for strings.
    let greeting = "Hello world!"; // type of &str
    // let another_greeting = greeting + " Glad to be here.";

    // Strings can span multiple lines. When compiled, new line characters are
    // generated.
    let a_longer_greeting = "Look at me,
    I'm a
    multi-line string";

    // Concatenate strings with `+`.
    // let name = "John " + "Wayne";
    // let email_subject = "Hi " + name + ", you're a valued customer";


    /*** > Option ***/

    /* Option is not a true primitive but it so common in Rust that it is
    reasonable to treat it as one. As mentioned, Rust was inspired by functional
    languages, and one great thing in those languages is the ability to
    reliably represent nothing, as with unit, and also the _possibility_ of
    something or nothing. */


    /*----------------------------------------------
    * Structs, Tuples, Enums, & Modules
    *-----------------------------------------------
    */

    /* As I mentioned, Rust is heavily inspired by OCaml. It tries to bridge the
    conceptual gap between academic and highly symbolic languages like ML and
    hardware-oriented languages like C. This means that the linguistic
    traditions of both languages sit sometimes uneasily next to one another.
    
    An excellent example of this is the distinction between "types", as
    illustrated above, and structs, tuples, traits, interfaces, and
    implementations. In C, these are different things because they represent
    different things in memory. In OCaml, there is no significant difference;
    they are all just types. A type is simply a description of the ways with
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
    denotes the entity created _by_ a struct usage. */

    /*** Unit Struct ***/

    /* One of the best aspects of OCaml was, since it separated the existence of
    an identifier and value in both the logic and the type structure, to have
    _unbound_ type identifiers that had no values. These are known as _abstract_
    types. In Rust, the language is a bit muddled and they instead refer to them
    as "unit structs". Unit again rears its head, but you will have to wait
    a little longer for a discussion on it.

    The below struct is a unit struct. Any entity using this struct does not
    need to know its structure just so long as usage of the type is consistent.
    Basically, programs can use symbols without knowing what they mean.
    
    The superpower of unit structs is to enable type-safe program structure
    experimentation and scaffolding. A developer can draw out a structure on a
    whiteboard, then code out the same structure, with unit structs being used
    to connect the entities. Later, when behaviors are figured out, the struct
    can be built out while maintaining strict type security across all of the
    struct's consumers. 
    
    Unit structs will be explored in detail in the sections on enums and
    implementations. */

    struct Kwyjibo;

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

    /*** Struct Methods ***/

    /* As mentioned, structs are just grouped data. They are not like classic
    "objects" in the sense of having behavior and data. To add behaviors, aka
    methods, to a struct, it must be "implemented". */

    struct Square {
        width: f64,
        height: f64,
    }

    impl Square {
        fn area(&self) -> f64 {
            self.height * self.width
        }
    }

    /*** Traits ***/
    
    let a_number = 42;
    let number_to_string = a_number.to_string();
    // let string_to_number = String::from(a_number);

    /* The different syntax represents where the method exists. If using dot
    syntax, the method exists on the instance and can potentially change. The
    double-colon syntax means it is a method on the type itself and will never
    change. A good analogy for those coming from other OO languages is to see
    the double-colon as calling static methods. The different syntax helps
    provide clarity as to where and how method calls work.
    
    The functionality seems duplicated, and for the use case here it is, but the
    underlying process is very different. These will be discussed later.  */

    // TODO: Cover this subject. Cover traits.

    /*** Tuples ***/

    /* Rust refers to tuples as a form of struct, which makes sense. If a struct
    is a set of primitive values in a shared space, sitting together in memory,
    then a classic tuple fits the bill. Just as with traditional structs */

    /*** Modules ***/

    /* Modules are similar in conception to modules in other languages. In
    comparison to other, object-oriented languages, a module also has passing
    similarities to classes. The primary purpose of modules is to hide structs
    and/or functionality from other parts of the program. */

    mod stuff {

        struct PrivateStruct {
            x: i32,
            y: i32,
        }
        // Notice how the individual keys can be public or private.
        pub struct PublicStruct {
            pub x: i32,
            pub y: i32,
        }

        // Only public structs can be exposed.
        pub fn get_thing() -> PublicStruct {
            PublicStruct {
                x: 42,
                y: 2001,
            }
        }
    }

    /* Neither PrivateStruct nor PublicStruct are accessible outside of that
    module by simply calling their names, even though this module is a child of
    the main() function. As such, this would fail.
    
    let new_thing = PublicStruct {
        x: 42,
        y: 2001,
    };

    The only thing that exists within the scope of main() is the module itself.
    As such, any usage of that module must call the module. */
    
    let new_thing = stuff::PublicStruct {
        x: 42,
        y: 2001,
    };

    /* Further, only public entities within the module can be consumed. In the
    above example, just as with basically every class-based language in
    existence, PrivateStruct cannot be consumed outside of the module. */

    /* A key difference from class-based languages is that a module is entirely
    defined by behavior. It contains no internal data. A function within a
    module can instantiate data, but it does not persist and only exists within
    the scope of the function. In the above example module, get_thing()
    instantiates a public struct and returns it via implicit return. */

    let i_got_a_thing = stuff::get_thing(); // type of PrivateStruct
    
    
    /*----------------------------------------------
    * Basic operators
    *-----------------------------------------------
    */

    /* The reason for putting basic operators so late into this tutorial is
    because they are somewhat supercharged in Rust. Instead of relying on fixed
    operators, custom evaluators can be written, thus allowing engineers to
    decide how operators such as `>` or `==` function. */

    /*** > Boolean ***/

    // Operators on the standard primitive types work as you would expect. 5 is
    // indeed larger than 2.

    let is_learning = true;

    let logical_and = true && false; // - : bool = false; 
    let logical_or = true || false;  // - : bool = true;
    let logical_not = !true;         // - : bool = false;

    let char_comparison = 'a' > 'b'; // - bool : false
    let number_comparison = 5 < 42;    // - bool : true

    println!("{} {}", char_comparison, number_comparison);

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
    };

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

    let big_obj = [10, 10000000, 10000000];
    let small_obj = [11, 1, 1];

    let big_array = big_obj > small_obj; // - : bool = false

    println!("{}", big_array);

    let compare_authors_1 = author1 == author2; // - : bool = false
    let compare_authors_2 = author1 == author1; // - : bool = true


    /* Comparing Values */

    // The equality operators work differently for values instead of structures.
    // Both `==` and `===` will become strict equality `===` when compiled to JavaScript.
    // Attempting to compare two different types will cause a compile error.

    let my_string_1 = "A world without string is chaos.";
    let my_string_2 = "A world without string is chaos.";

    "A string" == "A string"; // - : bool = true
    42 == 42;                 // - : bool = true
    // 42 === "A string"      // Error


    /*----------------------------------------------
    * Function
    *----------------------------------------------
    */

    /* Rust is deeply inspired by functional languages, so unsurprisingly its
    functions are distinctly different from many other languages, such as those
    from the C family. This difference was briefly discussed earlier where the
    semantic use of semicolons was pointed out.
    
    To reiterate, Rust has en explicit `return` statement as a concession to the
    C tradition, but it also has implicit final return as denoted by the lack
    of semicolon. In most cases, the final statement of a block being used as 
    the implicit return will be the ideal and idiomatic pattern.

    All evaluation blocks, and thus all functions, _must_ return something. If
    no final value is present, the block will return the special value `unit`,
    which will be discussed shortly.

    For example, the below function has one evaluation block: the if/else. As
    such, this is the return of the function. The if/else is composed of two
    evluation blocks that each return a value. Thus, the two booleans count as
    the final return of the entire function. */

    fn greater_than_42(x: i32) -> bool {
        if x > 42 {
            false
        } else {
            true
        }
    }

    /* It is important to note that, even though Rust has a return statement,
    it applies only to the function level, and not the level of general
    evaluation blocks. And only through the return statement can early return be
    achieved. To wit, while Rust allows a return statement, it restricts the
    semantics to avoid mixing up paradigms. Within evaluation blocks, only
    implicit returns are allowed, and Rust bars implicit early return.
    
    Let's break the function to illustrate. if the `false` is uncommented,
    the first false would lack a semicolon, and Rust's compiler would think that
    it is thus meant to be the block's return. But since there is a statement
    _after_ that, it knows that it cannot be the implicit return. It will thus
    throw a missing semicolon error.
    
    If the second if/else is uncommented, a similar problem arises. The second
    if/else becomes the implicit return of the function block, and thus the
    booleans contained therein become the return value of the entire function.
    The compiler knows that the implicit returns of the first if/else block are
    now not being caught by anything and will thus throw an error indicating
    that an explicit `return`, to thus break out of the function, was likely
    intended. This is why uncommenting the `return true` line does not throw an
    error, it will instead throw a warning of unreachable code.
    
    This illustrates how the need for an explicit return likely means that the
    function has been poorly designed. Composing a function of evaluation blocks
    that all return values, and having the function itself finally evaluate to a
    final value, should be the ideal pattern. */

    fn less_than_42(x: i32) -> bool {
        if x < 42 {
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

    /* Rust has two ways to declare functions which will again be very familiar
    to TypeScript developers. The first is the obvious way as illustrated above,
    but just like JavaScript and TypeScript, functions can be "anonymous",
    meaning that the function itself has no identifier, but is instead bound to
    an identifier. This allows a function to be passed around instead of
    simply called. The syntax is slightly different but likely very familiar to
    TypeScript developers who use fat arrow function syntax. */

    let sign_up_to_newsletter = |email: &str| -> String {
        format!("{} {}", String::from("Thanks for signing up"), email)
    };

    /* Of note, the standard function declaration syntax is actually called
    function _pointer_ syntax. That's right. The `fn` keyword is actually what
    is known as a "smart" pointer that allows performance _and_ safety. Smart
    pointers will be discussed later. */

    let get_email_prefs = |email : String| -> (String, [String; 2]) {
        let message = format!("{} {}", String::from("Update settings for"), email);
        let prefs = [String::from("Weekly News"), String::from("Daily Notifications")];

        (message, prefs)
    };

    // Call a function with the standard syntax.
    sign_up_to_newsletter("hello@rust_lovers.org");


    /*** Unit ***/

    /* You may have noticed in the above example of less_than_42, that if you
    uncommented the second if/else block, the specific error that was displayed
    was how () was expected, but a boolean was returned. In the previous
    section, I used the term "caught" when describing that the first if/else was
    returning something to nothing. That lack of a catcher for the evaluation's
    return means that Rust expected that block to return `unit`, or nothing.
   
    Unit is an interesting concept. It is the concept of a "thing" that is
    "nothing." It is different from `None` in that `None` is the state of
    nothing being where `Some()` could also have been. `Unit` is the state of
    expected nothing. It is similar to `void` in other languages. Unit can be
    declared with the `unit` type keyword or the `()` syntax. 
        
    From a mathematical perspective, it could be seen as the empty set, in
    that it is still a set, but it is a set of nothing. */
        
    // Unit's first use is in declaring functions that take no arguments.
    fn no_argument() -> String {
        String::from("I've got nothing")
    }

    /* All functions necessarily return something, so if there is no expression
    intended for return, such as in functions that only handle side-effects, then
    that function will return `unit`. Functions that return `unit` can be
    explicitly typed. */
        
    fn no_return(input: String) -> () {
        println!("I just print {}", input)
    }

    /* The above function expects to return nothing and will throw a compile
    error if anything is returned. */


    /*----------------------------------------------
    * Ownership
    *----------------------------------------------
    */

    /* At two points earlier in this tutorial, the concept of a return being
    "caught" by something was discussed. Now we can talk about what is doing the
    catching, which is the central concept to Rust's true party piece:
    
    Ownership.
    
    In Rust, every value has an "owner". This owner is the aforementioned
    catcher. An owner is also known as an identifier since only through the
    identifier can a value be accessed. */

    let catcher_in_the_rust = "Holden Caulfield";

    /* In the above, the value of "Holden Caulfield" is owned by the entity
    `catcher_in_the_rust`. They are "bound". In essence, the only permanent
    value assocaited with `catcher_in_the_rust` is the type of value that can be
    bound to it, in the above case `&str`. By using variable shdowing, the value
    that `catcher_in_the_rust` owns can change. */

    /*----------------------------------------------
    * Cargo
    *----------------------------------------------
    */

}

```
