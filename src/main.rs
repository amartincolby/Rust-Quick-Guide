/* These imports should be familiar to most. The double-colon syntax represents
the "path" to the entity. */
use core::num;
use std::array;
use rand::prelude::*;

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
famous _stack overflow_, from which the website gets its name.

The heap is precisely that: a big pile of memory space. The primary
differentiator between stack and heap entities is that anything on the stack
must be of known and fixed size. Anything that can change in size must exist on
the heap. The dangers of the heap include classics like null pointers and
memory leaks.

There are significant performance implications in the stack versus the heap, but
these are outside the scope of this tutorial. If you are coming from C or C++,
these considerations are not new to you. If you are coming from TypeScript, then
don't even worry about it. Rust written in even the most naive way is still an
order of magnitude faster that JavaScript. */

/*** A note on macros ***/

/* Throughtout this tutorial, you will see some commands appeneded with an
exclamation mark, like `println!`. This mark indicates that this command is a
"macro". Macros are _old_ in programming, having been first used in the 50's and
added to Lisp in 1963. A macro is something that evaluates a string of
characters and then interprets it. A programmer could genuinely implement their
own programming language within a macro. Macros are one of the constructs in
Rust that most programmers are not likely to have encountered. Macros are
incredibly powerful and complex. If they are included in this tutorial, they
will be at the very end. For the time being, just be aware that the exclamation
mark simply means that the tool in use is a macro. */


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
    * Items
    *----------------------------------------------
    */

    /* Items are entities that, whenever they are declared, they are analyzed and made global, even though their _visibility_ is restricted to the scope in which they were declared. They are "attached" to this scope like a key is attached to an object. This means that items can be referenced before they are declared. This is possible because items are entirely determined at compile time, meaning they exist before something like a function runs. In Rust parlance, items are "static" entities, with "dynamic" entities being their counterpart.
    
    The below is not a complete list of items, but represent the items important for this tutorial. All of these items will be discussed.
    
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

    From the perspective of semantics, items are the "hard" pieces of Rust code, the things that represent the structure through which the logic flows. As such, items cannot be created dynamically. To continue this analogy, if we liken a program to a building, what happens in the building can change over time, but what happens in the building should not determine how many floors the building has. */

    let value_to_be_enclosed = 42;
    let add_ints = |x: i32| x + value_to_be_enclosed;

    /*----------------------------------------------
    * Variables, functions, and bindings
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
    value. Values are not inherently mutable or immutable (again, they are
    just 1's and 0's in memory somewhere) it is whether the `mut` modifier is
    present that determines if the program will _treat_ that value as mutable or
    immutable.
    
    This may seem like an esoteric restatement of what was already said, but it
    is important to understand this relationship to better grasp the later
    discussion of Rust's party piece, ownership. */

    // In essence, the only permanent
    // value assocaited with `catcher_in_the_rust` is the type of value that can be
    // bound to it, in the above case `&str`. By using variable shdowing, the value
    // that `catcher_in_the_rust` owns can change. 

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
    run to determine. If you are coming from C, constants should be very
    familiar, only unlike C, the all-cap format is enforced. */

    const HALF_THE_ANSWER: i32 = THE_ANSWER / 2;
    const THE_ANSWER: i32 = 42;
    // const RANDOM_NUMBER: i32 = generate_random_int(); // Fails.

    /* Pay special attention to the constants HALF_THE_ANSWER and THE_ANSWER.
    Notice how THE_ANSWER is referenced _before_ it is declared. Constants are
    visible to everything within their scope, regardless of where they appear
    lexically. This is because they are one of the earlier-mentioned items. Since all items are created in global memory, they can be referenced anywhere within the scope in which they were written. */

    /* Constants cannot be shadowed within the same scope, but they can be
    shadowed in nested scopes. */

    // const THE_ANSWER: f64 = 3.14; // This does not work.

    let wrapper_scope = {
        const THE_ANSWER: f64 = 3.14; // This does work.
    };

    /* Constants provide a high-performance method for storing data of known, fixed size for sharing across parts of the application. */

    /*** Statics ***/

    /* Statics are the third and final way to declare a value in Rust. Static values are very similar to constants and many of the same rules apply. The difference is that a constant represents a value, while a static represents a memory location. As such, a key difference is that statics can be tagged as mutable. Rust requires any interactions with a mutable static to be flagged as unsafe since the value at that memory address can change unpredictably.
    
    By and large, constants will be used far more often than statics. The primary use case for statics over constants is when large amounts of data is being referenced. Constants are "inlined" during compilation. This means that everywhere where a constant is referenced, the value behind that constant replaces the reference. If the constant represents a lot of data, or if the constant is referenced many times, that could cause a huge increase in the size of the compiled binary. Statics put the data in one location only. */

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

    /* Take note that naked blocks can have no return value. */

    /* The list of reserved words can be found here:
    https://doc.rust-lang.org/reference/keywords.html

    Prefixing a variable name with an underscore creates a casual variable.
    These variables, if unused, will not trigger a compiler warning.
    
    Unused variable warnings only apply to block scopes and will not be
    raised on variables declared at the global or module level. 
    
    There are two kinds of unused variable. A suspicious unused variable is one
    that has been bound with `let`. An innocuous variable is one that has not.*/

    
    // To see the unused variable warnings below, comment out
    // #[allow(unused_variables)] near the top of this file.
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
    value name. */

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
    
    For the time being, a cursory explanation of pointers will suffice. */

    // The most common way to create a pointer is to bind the memory location of
    // a reference.

    let a_number_in_memory: i32 = 42;
    let a_pointer_to_a_number: *const i32 = &a_number_in_memory;

    /* Pointers are determined by the type signature of the identifier. */

    /*----------------------------------------------
    * Ownership & Borrowing
    *----------------------------------------------
    */

    /* Before the more complex elements of Rust are explored, it will behoove us
    to analyze Rust's party piece.
    
    Ownership.
    
    In Rust, every value has an "owner". An owner is also known as an identifier
    since only through the identifier can a value be accessed. When an owner
    goes out of scope, such as when a function completes, the value is "dropped"
    from memory. Rust does this automatically, but also supports an explicit
    "drop" command. Again, if coming from C++, all of this should be familiar
    through what is known as RAII, but unlike C++, Rust does this automaically
    and by default. If you write good, simple Rust, you will very likely never
    have to concern yourself with the cleanup process. C and C++ engineers will
    like that, but it is TypeScript engineers who should take most note, because
    it means that Rust can _feel_ garbage collected, making it much more
    approachable.
    
    That said, Rust's system of ownership can be a little confusing, and the
    compile errors that it produces can sometimes seem strange. But before that,
    let us go over the basics. */

    let catcher_in_the_rust = "Holden Caulfield";

    /* In the above, the value of "Holden Caulfield" is owned by the entity
    `catcher_in_the_rust`. They are "bound". That means that
    `catcher_in_the_rust` _owns_ that value. The value is a string literal,
    meaning a sequence of chars of known length, hard-coded into the binary, and
    which sits on the stack and not the heap. In this scenario, "Holden
    Caulfield" exists on the stack. Ownership gets interesting when using the
    heap. */

    // Using the String crate from the standard library allows us to create a
    // string of unknown size on the heap.
    let catcher_in_the_string = String::from(catcher_in_the_rust);

    /* The identifier `catcher_in_the_string` now stores a pointer to the heap
    which contains the string "Holden Caulfield". So here, things get
    interesting. */

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
    // Now, `successful_move` owns the value and trying to reference 
    // `catcher_in_the_heap` again would fail.

    /* Ownership tracking is Rust's safety net. Because remember, Rust clears
    memory when an identifier goes out of scope. See below. */

    let thats_what_i_want = String::from("Gimme money!");

    {
        let new_owner = thats_what_i_want;
        // After this, `new_owner` falls out of scope, and thus its memory is
        // cleared.
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
        // x is now destroyed.
    }

    memory_destroyer(istanbul);
    // "was Constantinople." is now destroyed.

    // Thus this will not work.
    // println!("{}", istanbul);

    /* Obvously, being able to only use a value once will not take you far. To
    allow values to be used without transferring ownership, Rust leverages the
    concept of "borrowing".
   
    This process is called borrowing because ownership is not transferred. The
    below is identical to the previous naked scope. */

    let respect = String::from("Find out what it means to me.");

    {
        let new_owner = &respect;
        /* Note the ampersand before the value, indicating that this is a
        reference. After this, only the reference is destroyed. The value is
        untouched. Because it is only a reference, multiple aliases to the
        original value are possible. */
        let another_new_owner = &respect;
    }

    /* Just as normal variable declarations are immutable by default, so are
    references. References have two layers of protection in that both the
    original value _and_ the reference must be tagged as mutable if the value is
    to be changed. When mutable, references are treated more carefully by the
    compiler. Any number of references can be _created_, but only one reference
    can be _consumed_. See below.*/

    let mut jeremiah = String::from("was a bullfrog.");

    {
        let new_owner = &mut jeremiah;
        let another_new_owner = &mut jeremiah;
        let yet_another_new_owner = &mut jeremiah;
        // Uncomment this line to see errors.
        // println!("{}", new_owner)

        // Meanwhile this succeeds because it was the most recent borrow.
        println!("{}", yet_another_new_owner);
    }

    /* The naked scope is now closed and all references are destroyed. */

    let new_owner = &mut jeremiah;
    println!("{}", new_owner);

    /* The mechanism performing these checks is called the "borrow checker." The
    point of the borrow checker is to prevent unexpected changes to values while
    the program runs. For those coming from something more free-wheeling and
    anarchic like JavaScript, this can initially feel overly restrictive, but
    it is _critical_ to Rust's value. Whole classes of errors are eliminated by
    this semantic decision. Learn it. Live it. Love it. */


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

    
    /*** > Option ***/

    /* Option is not a true primitive but it so common in Rust that it is
    reasonable to treat it as one. As mentioned, Rust was inspired by functional
    languages, and one great thing in those languages is the ability to
    reliably represent nothing, as with unit, and also the _possibility_ of
    something or nothing. */

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
    implications. Rust does not do this and a slice must have data behind it. */

    // Slices are created by providing bounds with indices.
    let slice_of_three = &array_of_five[1..4];

    // Slicing the entire array can be done by omitting the values.
    let slice_of_five = &array_of_five[..];

    /* Slices are more generic in Rust than other languages, and as such they
    can be used with more than just Arrays. Most importantly, slices can be
    used with strings, which will be discussed now. */

    /*** > Vector ***/

    /* For Go developers accustomed to slices and JavaScript/TypeScript
    developers accustomed to arrays, Rust has a "vector." Vector is not a true
    primitive and is instead part of the standard library. Vectors are typed
    lists, similar to arrays, but are dynamically sized and exist in the heap.
    The Rust documentation calls them a "collection." Vectors are essentially
    syntactic sugar over implementations and as such rely on "generics."
    Generics are common in other languages like TypeScript or C++, so this
    should not be too alien for anyone, but generics will also be discussed
    later. */

    // The generic type <i32> creates an empty vector that stores i32's.
    let mut viktor: Vec<i32> = Vec::new();

    // Vectors can infer types as well when created with the vector macro.
    let v = vec![42, 2001, 314, 1999];

    // Vectors behave similarly to arrays.
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

    // Borrowing any index from the vector will lock the entire vector.
    // This is because when a vector is destroyed, all of its elements are too.
    let index_1 = &viktor[1];
    viktor.push(1999);
    // println!("{}", index_1); // Causes borrowing error.

    /* Slice syntax on vectors is identical to arrays. */
    let rogue = &v[..];
    let johnny = &v[1..3];

    /*** > String & str ***/

    /* Like vectors, strings in Rust are not true primitives in the sense that
    a primitive is a thing of known, fixed size. They are like C strings in that
    they are best described as an array of characters, with each character being
    the true primitive, and are again classified as a collection.
    
    The fundamental type, to wit the type that is part of the language itself,
    is `str`, which is a sequence of chars _somewhere_ in memory. It could be on the stack, it could be on the heap. What is important to know is that the compiler does not know the length of this sequence at compile time. As such, when creating str entities, they are almost always _referenced_. As such, they usually exist in slice form, to wit &str. The string slice does have a known length.
    
    To illustrate, "ABCDEFG" are chars sitting next to each other in memory. A
    slice can represent all of them, some of them, or none of them. Rust does
    not have a type for representing the sequence of chars itself. */

    // Use double quotes for strings.
    let greeting = "Hello world!"; // type of &str

    /* Notice how the type above is a _reference_. That is because, since Rust has no true string primitive, the identifier `greeting` is actually a reference to a collection of chars hard-coded into the binary and sitting on the stack. This means string slices never own the memory in which the chars exist. */

    // Strings can span multiple lines.
    let a_longer_greeting = "Look at me,
    I'm a
    multi-line string";

    // The key difference is that str is of known length, while String is not.

    // The `String` type that was used in earlier examples to create a string on the heap is actually part of the standard library and is fundamentally a wrapper around `str` that provides helpful functionality. Because of the common usage of `String`, the two types are often confused in conversation, with people using the term "string" to refer to either `String` or `str`.

    /* To reiterate, using the String crate from the standard library creates a sequence of chars on the heap. This string can be added to and reduced, but as it is a collection, interactions with it are similar to a vector. Indeed, this is because under the covers, String _is_ a vector. */

    let mut heap_of_chars = String::from("A few of my favorite things: ");
    
    heap_of_chars.push_str("raindrops on roses ");
    heap_of_chars.push_str("whiskers on kittens ");

    /* Because   */

    let string_slice = &heap_of_chars[1..3];

    /* String can also be coerced into &str via the type of a function parameter. */

    fn str_coercer(s: &str) {
        println!("{s}")
    }

    // In the below, a &String is being coerced into a &str.
    str_coercer(&heap_of_chars);


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

    /* Implementations are tied to the struct, meaning that anywhere that
    `Square` is visible, the `area` method is also visible. */

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
    then a classic tuple fits the bill. I am not a fan of it because I see a key element of structs as being key access Just as with traditional structs, names for tuples use CamelCase. Unlike traditional structs, they are accessed by order instead of key. None of this should be new to basically every programmer on Earth. */

    struct Coordinate(i32, i32);

    let treasure = Coordinate(42, 2001);

    /*** Enum ***/

    // If you have any experience with OCaml, you will recognize enum as being equivalent to union, sometimes called discriminated union or tagged union. Enum is a type that defines an identifier that can be bound to any of the enum's consituent types. For example, a value could be a string _or_ a 32-bit integer, an enum allows that to be represented. The Rust type checker will then ensure that any code that consumes an enum must handle all possible cases.

    struct Ford;
    struct Toyota;
    struct BMW;

    enum Car {
        Ford,
        Toyota,
        BMW,
    }

    let my_car: Car = Car::BMW;

    /* Each possible state of an enum is called a variant, since they are the varied ways in which an enum can exist. The variants are distinguished from one another via the aforementioned tag or discriminant. The above Car enum has implicit discriminants, integers starting with 0. Thus, Ford is 0, Toyota is 1, and BMW is 2. This should be familiar to anyone coming from TypeScript, C, C++, or really any number of languages. Discriminants can be any integer. */

    enum Truck {
        F150 = 420,
        Silverado = 42,
        Tundra = 69,
    }

    /* Notice in the above enum how the three trucks were not declared as structs before. An enum necessarily contains other types, meaning that any variant that has not been previously declared is automatically declared as a unit struct. */
    

    /*** Union ***/

    /* The reason for why Rust did not simply take OCaml's terminology is because Rust also took the union type from C. Unlike the enum, which can be understood separate from the machine state, a union requires an understanding of memory. A union is a section of memory that can store any of the value types as listed in the union. This means that the memory consumed will be whatever is the largest type. */

    union IntUnion {
        small_int: i8,
        large_int: i64,
    }

    /* In the above union, 64 bits of space will be reserved because one possible value is a 64-bit integer.
    
    Chances are you will never use a union. Interacting with them is part of what is considered unsafe Rust and an enum will be a better choice in almost every case. The primary use case for unions is when extreme memory efficiency is needed and the safety of enums consumes memory. */




        
    /*----------------------------------------------
    * Generics
    *-----------------------------------------------
    */

    /* If you are coming from C++ or TypeScript, generics should be very
    familiar to you. Go only recently introduced them, but they are common
    across most typed languages. Generics are essentially just entities that
    can accept different types, and the type signature of that entity is
    different based on those types. The syntax for this is broadly similar to
    TypeScript, so it should be easy to pick up. */

    struct LinkedList<T> {
        head: LinkedListNode<T>,
    }
    
    struct LinkedListNode<T> {
        value: T,
        next: Option<Box<LinkedListNode<T>>>,
    }


    /*** Type Aliases ***/

    /* Rust enables aliasing of types to different names. Aliases are confusingly declared with the `type` keyword. This is one of the few areas of Rust's syntax with which I strongly disagree. `type` is from OCaml and they should have left it there. Aliases are intended to enable semantic naming of broad, generic types. For example, below a linked list representing stops on a trip can have the type aliased so the type of the list itself provides semantic information. */


    type TripStops = LinkedList<String>;


    /*----------------------------------------------
    * Modules & Crates
    *-----------------------------------------------
    */

    /* Modules are similar in conception to modules in other languages. In
    comparison to other, object-oriented languages, a module also has passing
    similarities to classes. The primary purpose of modules is to hide types
    and/or functionality from other parts of the program.

    Crates are a unit of compilation. That is to say that a crate is the result
    of a compilation. There is some syntactic similarities when interacting with
    modules or crates. The imports at the top of this file reference a crate,
    then use double-colons to traverse modules within that crate to reach
    entities. The root of a crate is also a module.

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
    actually part of its type.  */

    let big_obj = [10, 10000000, 10000000];
    let small_obj = [11, 1, 1];

    let big_array = big_obj == small_obj; // - : bool = false

    println!("{}", big_array);

    let compare_authors_1 = author1 == author2; // - : bool = false
    let compare_authors_2 = author1 == author1; // - : bool = true


    /* Comparing Values */

    // The equality operators work differently for values instead of structures.
    // Attempting to compare two different types will cause a compile error.

    let my_string_1 = "A world without string is chaos.";
    let my_string_2 = "A world without string is chaos.";

    let compare_strings = "A string" == "A string"; // - : bool = true
    let compare_integers = 42 == 42;                // - : bool = true
    // let compare_number_string = 42 == "A string" ;     // Error


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
    
    Let's break the function to illustrate. if the first `false` is uncommented,
    it false would lack a semicolon, and Rust's compiler would think that it is
    thus meant to be the block's return. But since there is a statement _after_
    that, it knows that it cannot be the implicit return. It will thus throw a
    missing semicolon error.
    
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

    sign_up_to_newsletter("hello@rust_lovers.org");

    /* Of note, the standard function declaration syntax is actually called
    function _pointer_ syntax. This is because items like function pointers are only ever instantiated once, in a single place, meaning that any calls to that function do so through a pointer to the function's static location. */

    /* As mentioned, function pointers in Rust are notably different from functions in many other languages, and one of the most significant differences, if not the most, is that function pointers cannot access values declared outside of their scope. This is known as "capturing" a value. If you are coming from TypeScript, the common term is "enclosing," to wit you are writing a closure. */

    let outer_var = 42;

    fn normal_function() -> i32 {
        let inner_var = 42;
        inner_var // This works.
        // outer_var // This does not.
    }

    /* The above is not possible because a `let` binding is part of the "dynamic" environment of the program. The dynamic environment is the part of the program that can change based on how the program runs. The "static" environment is the part of the program that is the same whenever a section of code is run. Because function pointers are static items, they do not exist on the same level as let declarations.
    
    If you are coming from TypeScript or JavaScript, you may recognize this as sounding like hoisting, and while that is not entirely wrong, it is not entirely right. Functions do not get moved to the top of a scope, as they do in JavaScript. Function pointers, like all items, are lifted into a different realm. That said, the concept of hoisting gives us a good illustration for why Rust works as it does.

        displayMessage();

        let message = "a message for you";

        function displayMessage() {
            console.log(message);
        }
    
    In the above JavaScript code, a function can be used before its declaration. But this code will fail because the `displayMessage` call is relying on `message`, which is declared after the call. If Rust tried to allow the usage of function pointers with outside values, the function would not be able to know where to find this value. Thus, Rust simply prevents this.
    
    There are many uses for this pattern, though, and Rust */

    const OUTER_CONST: i32 = 42;

    let test1 = |x: i32| x * 2;
    let test1 = |x: i32| x * 2;

    /* You will often see syntax like the above in Rust code. If coming from
    TypeScript or JavaScript, it is semantically similar to:
    
    let test1 = (x : number) => x * 2;

     */

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

    

    /*----------------------------------------------
    * Cargo
    *----------------------------------------------
    */
}

/* This content is part of a section in the above function. Do not read it
separately.

These modules are not nested inside of a function. They are in the base scope of
the file and thus exist on the module level. They can thus see each other. */

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
