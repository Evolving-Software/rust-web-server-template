
// This file is used to load the evolving.js library
// It is loaded by the index.html file

class Evolving {
  constructor() {
    this._version = "0.0.1"
  }

  get version() {
    return this._version
    }
    
    // Add more methods here
    // a wrapper for the fetch API
    // https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API/Using_Fetch
    // Usage:
    // const data = await fetch(url, options);
    async fetch(url, options) {
        const response = await fetch(url, options);
        const data = await response.json();
        return data;
    }
    
}

console.log("Using Evolving.js Version: " + new Evolving().version);

document.getElementById("fetch-data").addEventListener("click", async () => {
    const data = await new Evolving().fetch("https://jsonplaceholder.typicode.com/todos/1");
    console.log(data);
});

