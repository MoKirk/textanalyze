//use textanalyze::*;

/*
 *
 * The textanalzyer has several parts
 *
 * data:
 * those are possible lists used by the various linters
 *
 *
 * input :
 * - loads file / files / parses input
 *
 * text:
 * - is the struct with the file data
 *
 * linters:
 * - diffrent linters are available to parse the text
 *
 * output
 * - How to print the output from the input
 *
 * TODO:
 * - [x] create a text struct with line and sentence parser
 * - [x] create an input which returns the text from a file
 * - [ ] create an output which gets the text
 * - [x] create a first linter
 * - [ ] how can I make linter multilanguage?
 * - [ ] Check: Is the linter output okay or is it too complicated?
 * - [ ] Check: I dont like the result type currently, that could be made better
 * - [ ] Check: Maybe the linter should be run on sentences, not on text
 * - [ ] How can i make the linters more flexible?
 */
fn main() {

    /*
     * Should look like this
     *
     * let analzyer = Textanalzer::for_file(file)
     * analyzer.run();
     * analyzer.print_output()
     *
     * analyzer[input] = [file input] /// maybe only in the for file method
     * analyzer[text] = [holds the text]
     * analyzer[linter] = [array of linter]
     * analyzer[result] = [result of all the linter -> will be pushed into the output]
     * analyzer[output] = [ouptut]
     *
     */
}
