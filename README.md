This repo is a demonstration of a bug/issue i have with "egui_kittest".

### How to reproduce
#### On Nix
If you are using nix, then use the "flake.nix" & "flake.lock" file to set up the environment for this example.

#### Other
The only requirement is to have rust "1.85" installed.

---

Run "cargo test" to run the tests, with the output images generated in "./tests/snapshots"

### Expectation
I expect there to only be one input per textbox.

#### One by one
This is the output from trying to input text into the first empty text box three times.

![](./tests/snapshots/one_by_one.png)

#### All at once
This is the output from adding text to each box during the same frame.

![](./tests/snapshots/all_at_once.png)

### Keyboard Inputs
After the first textbox has been found, you can use the "TAB" key to navigate between text inputs (at least i can manually when running the program).
In this test i tried emulating that.

It seemed to work initially, but i could only navigate between text boxes with "TAB", and not use the "press_key" method on harness to type text into the textboxes.
(The "First" text is in the first textbox is me using the same code as [Sanity Check](<README.md#Sanity Check>) to get focus on the first textbox).

![](./tests/snapshots/keyboard_input.png)


### Sanity Check
As a sanity check i tried just adding text to the first text box, which does work as expected.

![](./tests/snapshots/single_works.png)

