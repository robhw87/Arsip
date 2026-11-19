---
title: About
category: About
tags: self-taught, rust, career-pivot, data-analysis, iso-9001
---

A self-taught engineer's path from eight years on a production floor
to data analysis, internal software, and now Rust.
Why I build ISO 9001 helper tools, and how I think about
designing software that stays easy to change

---

## About

Welcome, let me introduce myself. My Name is Robby Hadi Widjaja.
I'm a self-taught software engineer
moving toward Rust-heavy work.
I came to software from the factory floor, and that shapes how I build:
I care about tools that solve real operational problems and stay easy to change.

### From production floor to data

I spent eight years as a production supervisor in a
factory running under ISO 9001 and IATF 16949.
The title said production, but much of my real work was data analysis.
I lived in Excel: large workbooks with formulas and pivot tables,
often several pivots in one file, where the result of the first fed the second,
and so on until the process was finished.

It worked until the data grew. With hundreds of thousands of rows,
Excel became slow, fragile, and hard to trust.
I started looking for better tools, which led me to SQL,
and then to the Google Data Analytics Certificate on Coursera in 2023.
I enjoyed it more than I expected.
It opened the door to the world of IT, and from data I moved toward the web.

### Into software

After eight years on the production floor,
I proposed moving into IT at the same company.
My data analysis work is what made the case,
since the company had already seen what I could do with its production data.
Web development was the part I still had to learn,
so I learned it on the job, building internal applications next to our ERP:
automated reporting, SOP integration, SPC tooling,
and other gaps the ERP can't fill.

That work happens in a legacy PHP codebase,
so I've had to learn how to improve a system
incrementally while people depend on it.
My main question now is how to design building blocks
that are easy to refactor later.

### Why Rust

Two things drew me to Rust: efficiency and security.
I like software that is both efficient and effective,
and I rank security alongside quality,
because I want confidence that
what I ship won't fall over from bugs or become a liability.

Rust's ownership model rules out whole classes of
memory-safety bugs at compile time,
and its type system lets me encode business rules so
invalid states are hard to represent.

Rust doesn't solve everything
(logic errors, injection flaws, and weak authentication are still my job),
so I pair it with input validation and dependency auditing [e.g. `cargo audit`].
Learning Rust is also how I'm learning architecture and design, not only syntax.

I've used Rust in production for scheduled jobs
[describe briefly, e.g. what they do].
The ISO 9001 helper tools on this site are my first larger Rust codebase.

### What I'm building here

This site is built with Axum, Maud, PostgreSQL, and Redis,
with a small custom CMS that turns Markdown files into
database metadata and cached content.
On it I'm building ISO 9001 helper tools
and writing up every decision,
using type-driven and domain-driven design where they help,
and saying so when they don't.

Start with [link to the first series post]. The code is on [GitHub link].

### ISO 9001 and consulting

I also help small organizations make sense of ISO 9001
[your actual offer, e.g. gap review, documentation cleanup, audit preparation].
My background is eight years of hands-on implementation,
not only audit preparation.
[Formal training or certificates, if any.
If none, leave this line out rather than imply them.]

If you'd like to talk, you can reach me at [contact link].
