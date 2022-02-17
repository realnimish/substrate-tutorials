# training-substrate - 02 - pallet-extrinsics

A user that would like to make transactions on the blockchain call dispatchables functions defined in the different pallets of the runtime. Theses calls are then stored as extrinsics in the blockchain. As runtime developer you can update pallets and add more of theses callable function depending on your needs.

In this exercice, you will work on the same pallet as the previous exercice, the flipper pallet, and you will have to add a new callable function that should allow a user to remove the content of the storage. Also, you will have to make tests to ensure of the good behaviour of the function.

## Setup
* Clone [the training-substrate repository](https://github.com/rusty-crewmates/training-substrate).
* Create a branch from the ```pallet-extrinsics``` one and call it <YOUR_FIRST_NAME>/pallet-testing.

## To do
You will work on the ```pallets/flipper/src/lib.rs``` and the ```pallets/flipper/src/tests/.rs``` files.
1. Make a callable function ```remove_value()```, it should clear the storage and emmit an event to inform the user that it's been done. The function should return a DispatchResult and you will have to handle errors if some could occurs.
2. Make some tests for your new function.

> Don't forget to comment your code

## Some links
* Awesome Rusty : https://github.com/rusty-crewmates/awesome-rusty
* Storage : https://docs.substrate.io/v3/runtime/storage/
* Events and Errors : https://docs.substrate.io/v3/runtime/events-and-errors/
* Storage Value Trait : https://docs.rs/frame-support/latest/frame_support/storage/trait.StorageValue.html#tymethod.kill

## Ensure everything is ok
```cargo check && cargo test && cargo bench && cargo fmt && cargo build && cargo build --release```

## Push your work
Commit like this : ```[<YOUR_INITIALS>] <branch-name> / <short description>```<br/>
Example : ```[SG] pallet-testing / flipper pallet implementation + tests```
