# preq-ngin

Automated prereq classification! Since we want to be able to programatically interact with prereqs, we need a way to convert them from non standardized strings into rich data objects. Rich data objects allow us to search, filter and ask questions. Whereas, with strings you cannot do much except match keywords (which fails in reality where people mis-spell or write slightly different setences with same intent).

A simple and common question for a student may be - can I take `MAT 242` if i've taken `MAT 100` and `MAT 200`? 

#### Interface

<img height="700px" src="https://raw.githubusercontent.com/drbh/preq-ngin/master/images/demo.mov.gif" alt="demo gif">

### How it works

We currently do not have a good way to answer the above question. You can query for `MAT 242` and read the requirements string and check if those classes are included, but this requires human interaction (no good!)

Best case, you should be able to ask the computer. I got `A` and `B` can I take `C`? So we do just that! 

The system is simple and relies on safe fast technology like Regex, Rust and even Sublime Text's searching algo. 

Basiclly we get each requirement string -  break it into it's individual requirements. Then we attempt to classify each of those requirements. This process makes me think of fitting pegs into a box. 

![fitting](https://raw.githubusercontent.com/drbh/preq-ngin/master/images/square-peg.jpg)

Where in out case, we just try a bunch of fuzzy text matching untill we choose a shape that is the best fit. We tag a requirement with as many of the known classification if they meet a suffiently high enough similarity score.

After we've tagged all the requirements. We want to intelligently pull all of the mentioned courses out. This is a diffcult matching problem as well since people have encoded classes in various formats. 

## Included Apps

#### Scrape
This script is not included and the orignal datasource should be used in future iterations.
```
cargo run --bin scrape
```

#### Parse
This app allows us to parse all of the HTML pages we previously grabbed. It extracts the pre req text and course name for all `12035` classes. 
```
cargo run --bin parse
```

#### Classify
This app does the heavy lifting and works through a few steps to determine which pre known prereq group it most likely falls under. 

eg. `a min of 45 hours` should be correctly classifed as a `min hour requirement`. We repeat this for all known cases and can add new ones as they arise.  

Next the classifier attempts to intelligently extract course names. This is important when classes are not encoded in the tradtional `SUB NBR` format for instance, many times there is a single subject and many numbers eg. `SUB_A NBR NBR NBR SUB_B NBR NBR` and the classifier will unwrap this string into their individual courses 
```
SUB_A NBR
SUB_A NBR
SUB_A NBR
SUB_B NBR
SUB_B NBR
```

Lastly we format the data into JSON and write this to a file that can be used for various prereq checking applications!

```
cargo run --bin classify | jq > results.json 
```

#### Serve

Finally we can use the data for 3 simple applications:

1. How was a course classified?
2. Where is a course mentioned in pre reqs
3. If I've taken X,Y.Z classes, can I take class B?

```
cargo run --bin server
```

