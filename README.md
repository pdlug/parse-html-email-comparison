# Parsing HTML email - a learning experiment in Rust and Go

This was a quick hack over the weekend, none of this code should be taken as production grade or as idiomatic code in Rust or Go. I've been learning Rust for a bit and it's been an interesting and frustrating experience. I was recently encouraged to take a look a Go. I hadn't tried Go in years, the aesthetics of the language had put me off. I thought it looked like what C should have been in 2000 but not a "modern language". So, I decided to give it a fair chance and try Go, Rust, and TypeScript. Here are the results.

## The challenge

I've been building a small app to manage my subscriptions to email newsletters (more on that soon). As part of that, I downloaded a large number of email newsletters from my Gmail and want to extract links. This will be useful for spotting trends, search, and a few other applications. So the initial challenge is to load a JSON file of exported messages with this format:

```json
[
  {
    "id": "fc62fdcd-e8ce-4f54-a7c3-d28a56d3d663",
    "fromEmail": "news@thehustle.co",
    "fromName": "The Hustle",
    "htmlBody": "<!DOCTYPE html PUBLIC \"-//W..."
  },
  ...
]
```

Then parse each HTML body, and write out a file containing the links (`<a href="">...</a>`) extracted from all the messages.

This wasn't a test of: "what is the optimal way to parse HTML emails and extract links?". It was more a test of: "what does the naive out of the box solution in each language look like and how does it perform?".

## Running

Instructions below to run each version. Each version will output the runtime for a single invocation.

TS

```shell
cd ts
npm i
ts-node src/parse.ts
```

Go

```shell
cd go
go build
./go-email-parser
```

Rust

```shell
cd rust
cargo build --release
./target/release/rs-email-parser
```

## Observations

### Typescript

This one was the easiest to write since it's my strongest language at the moment. The only challenge was finding a decent HTML parsing library. [JSDOM](https://github.com/jsdom/jsdom) choked on the stylesheet references in some of the emails. That was fine because it was the wrong tool for the job anyway. I don't need a full featured browser DOM implementation, just something that can parse the HTML and handle any oddities. I found [linkedom](https://github.com/WebReflection/linkedom) which was fast, ergonomic, and handled everything I threw at it. Writing this took about 5 minutes since it's something I do in TS all the time.

### Go

Well, that was easy.

With no prior knowledge of Go beyond writing "Hello, World!" a few years ago, I was able to read the docs, Google some things, and get this working in about 10-15min. Everything worked as expected on the first try and was clearly much faster than the TypeScript solution.

### Rust

Well, that was hard.

Rust continues to make me feel like I'm just not smart enough to program in it. Getting a working solution was a battle at every step along the way. It took lots of reading Stack Overflow and trying to see what the compiler let me get away with to get it working. This easily took 45min to an hour which was the longest development time of any solution. Looking back on it, I still barely understand all the pieces.

## Benchmarks

These are not scientific at all. Perhaps the only lesson here is what out of the box performance looks like for someone hacking at a language the first time with no prior knowledge of any of the performance implications of their actions.

| Language   | Avg. Time |
| ---------- | --------- |
| TypeScript | 2688ms    |
| Go         | 796ms     |
| Rust       | 494ms     |

## Conclusion

Again, I feel a disclaimer is necessary here. This is not a fair benchmark because the time is dominated by HTML parsing and I have no idea what the relative optimization levels of the parsing solutions in each language.

Go was the surprise here. I expected Rust to be the fastest and Go to be faster than TS but I didn't expect how fast and easy it would be to write code in Go. Based on my experience here, it seems pretty easy to rewrite some performance critical code in Go vs. TS and achieve a pretty big boost. If you're deploying to AWS Lambda or similar solutions that charge for CPU + memory you'd see some pretty big cost savings.
