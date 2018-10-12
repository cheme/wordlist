const js = import("./parity_wordlist.js");

js.then(js => {
  console.log(js.random_phrase(18));
});
