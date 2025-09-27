Fix conversion code
Do parent code
Do child code
Make parent and child Vec<Rc<Refcell>> or maybe Vec<Arc<Mutex>> instead of a Vec of Uuids?
    See linked list implementation in rust
    This may be best memory wise though as Uuid is just a number
Finish making tests for nutrient
Make tests for nutrient amounts
Consider removing Add and Sub impl functions from nutrition amounts
    It can crash the program if the nutrients are not the same
    Make it a function with error handling instead?
Consider adding an ordering impl function
