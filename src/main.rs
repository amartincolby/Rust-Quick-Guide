const _GREETING: &str = "Stay awhile. Stay foever.";

/* Comment blocks start with slash-star,
   and end with star-slash. */
// Line comments begin with a double slash.

/* The below is the main function which all Rust applications have. This quick
    guide does not cover the structure of Rust applications. It focuses on the
    syntax and semantics. For the purposes of this guide, it is sufficient to
    know that Rust applications have a main function.

    The syntax structures above the function are called attributes. They allow a
    developer to specify how the function is the be handled by the compiler. In
    these examples, three linting settings are being disabled. More attributes
    will be covered later. */

#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
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

    // Functions will not infer parameter and return types.
    // fn add_ints(a, b) { a + b } // This will fail to compile.
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
    location in the computer's memory.
    
    This form of value access is extremely performant since it directly uses the
    computer instead of passing through the symbolic realm. But it is also very
    dangerous. This is why pointer usage is associated with what is known as
    "unsafe" Rust.
    
    Except for library writing, unsafe Rust is something that the average
    developer will never do. "Safe" Rust's performance is already so good that
    unsafe Rust is unecessary in all but the most demanding situations. Unsafe
    Rust will be explored more fully later.
    
    Because pointers in Rust are deeply connected with Rust's patterns of
    memory management, a full explanation this early into the tutorial is not
    tenable. A full explanation of pointers will happen later. */

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
    
    /*** > String ***/

    /* Strings in Rust are not true primitives in the sense that a primitive is
    a thing of known, fixed size. They are like C strings in that they are best
    described as an array of characters, with each character being the true
    primitive.
    
    The fundamental type, to wit the type that is part of the language itself,
    is `str`, which is called a string slice. The `String` type is actually part
    of the standard library and is best described as a wrapper around `str` that
    provides helpful functionality.
    
    A string slice is a slice, and a slice is a window into a contiguous
    sequence of entities in memory. */

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

    /*** > Char ***/

    // Use single quotes for a char. Chars compile to an integer between 0 and
    // 255 and thus do not support Unicode or UTF-8.
    let last_letter = 'z';


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
    let string_to_number = String::from(a_number);

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
        use stuff::PrivateStruct;


        struct PrivateStruct {
            x: i32,
            y: i32,
        }

        pub struct PublicStruct {
            x: i32,
            y: i32,
        }

        pub fn get_thing() -> PrivateStruct {
            PrivateStruct {
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
    instantiates a private struct and returns it via implicit return. */

    let i_got_a_thing = stuff::get_thing(); // type of PrivateStruct
    
    
    /*----------------------------------------------
    * Basic operators
    *-----------------------------------------------
    */

    /* The reason for putting basic operators so late into this tutorial is
    because they are somewhat supercharged in Rust. Instead of relying on fixed
    operators, custom evaluators can be written, thus allowing engineers to
    decide how operators such as > or == function. */

    /*** > Boolean ***/

    // A boolean can be either true or false.
    let is_learning = true;

    let logical_and = true && false; // - : bool = false; 
    let logical_or = true || false;  // - : bool = true;
    let logical_not = !true;         // - : bool = false;

    'a' > 'b'; // - bool : false
    5 < 42;    // - bool : true

    /* Equality */

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

    /* Any attempt at using greater-than or less-than vis-à-vis structures will
        trigger a structural comparison. The comparison will return a boolean as 
        soon as a difference is discovered and will not continue to do a complete
        comparison. */

    let big_obj = [10, 10000000, 10000000];
    let small_obj = [11, 1, 1];

    big_obj > small_obj; // - : bool = false

    /* Because 10 and 11 are different, and 10 is smaller than 11, false is returned
        even though the next two values are much larger. */

    /* Referential, or physical, equality, using the triple-glyph operator, compares
        the identity of each entity. */

    author1 == author2; // - : bool = false
    author1 == author1; // - : bool = true


    /* Comparing Values */

    // The equality operators work differently for values instead of structures.
    // Both `==` and `===` will become strict equality `===` when compiled to JavaScript.
    // Attempting to compare two different types will cause a compile error.

    let my_string_1 = "A world without string is chaos.";
    let my_string_2 = "A world without string is chaos.";

    "A string" == "A string"; // - : bool = true
    42 == 42;                  // - : bool = true
    // 42 === "A string"       // Error

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
    simply called. The syntax is slightly different. */
    let sign_up_to_newsletter = |email: &str| -> String { format!({-"{} {}", "Thanks for signing up ", email}) };

    let getEmailPrefs = (email) => {
        let message = "Update settings for " ++ email;
        let prefs = ["Weekly News", "Daily Notifications"];

        (message, prefs); // Implicitly returned
    };

    // Call a function with the standard syntax.
    signUpToNewsletter("hello@reason.org");


    /*** Unit ***/

    /* You may have noticed in the above example if you uncommented the second
    if/else block the specific error that was displayed was how () was expected,
    but a boolean was returned. In the previous section, I used the term
    "caught" when describing that the first ef/else was returning something to
    nothing. That lack of a catch for the evaluation return means that Rust
    expected that block to return `unit`, or nothing.
    */

    /* Reason has a special type called `unit`. It is the concept of a "thing"
        that is "nothing." It is different from `None` in that `None` is the state
        of nothing being where `Some()` could also have been. `Unit` is the state of
        expected nothing. It is similar to `void` in other languages. Unit can be
        declared with the `unit` type keyword or the `()` syntax. 
        
        From a mathematical perspective, it could be seen as the empty set, in
        that it is still a set, but it is a set of nothing. */
        
    // Unit's first use is in declaring functions that take no arguments.
    let noArgument = () => { "I've got nothing" };

    /* All functions necessarily return something, so if there is no expression
        intended for return, such as in functions that only handle side-effects, then
        that function will return `unit`. Functions that return `unit` can be
        explicitly typed. */
        
    let noReturn = (input) : unit => { print_endline("I just print " ++ input) };

    /* The above function expects to return nothing and will throw a compile error
        if anything is returned. */


    /*** > Labeled Paramters/Arguments ***/

    type position = {
        posx : float,
        posy : float
    }

    // Parameters can be labeled with the `~` symbol.
    let moveTo = (~x, ~y) => {posx : x, posy : y};

    // Any order of labeled arguments is acceptable.
    moveTo(~x=7.0, ~y=3.5);
    moveTo(~y=3.5, ~x=7.0);

    // Labeled parameters can also have an alias used within the function.
    let getMessage = (~message as msg) => "Message for you, sir... " ++ msg;

    // Labeled parameters support explicit typing.
    let logMessage = (~message: string) => {
        print_endline(message);
    };

    let logAnotherMessage = (~message as msg: string) => {
        print_endline(msg);
    };

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

}
