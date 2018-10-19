const js = import("./parity_wordlist.js");

js.then(js => {
  window.tested_wasm = js;
  var ph_size = 18;
  var ph = js.random_phrase(ph_size);
  console.log("generate phrase: ");
  console.log(ph);
  console.log("validate phrase should not throw");
  console.log(js.validate_phrase(ph, ph_size));
  console.log("validate phrase with wrong size minus");
  console.log(js.validate_phrase(ph, ph_size - 1));
  console.log("validate phrase with wrong size plus");
  try {
    js.validate_phrase(ph, ph_size + 1);
  } catch (e) {
    console.log("catch exception : ", e);
  }
  console.log("validate 'dummy' phrase");
  try {
    js.validate_phrase("test bd dd", 3);
  } catch (e) {
    console.log("catch exception : ", e);
  }
 
});
