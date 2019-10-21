# preq-ngin




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

#### Interface

[image](https://raw.githubusercontent.com/drbh/preq-ngin/master/images/interface.png)


