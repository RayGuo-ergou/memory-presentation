---
# try also 'default' to start simple
theme: seriph
# random image from a curated Unsplash collection by Anthony
# like them? see https://unsplash.com/collections/94734566/slidev
background: https://cover.sli.dev
# some information about your slides, markdown enabled
title: Memory Safety
info: PY presentation on memory safety
# apply any unocss classes to the current slide
class: text-center
# https://sli.dev/custom/highlighters.html
highlighter: shiki
# https://sli.dev/guide/drawing
drawings:
  persist: false
# slide transition: https://sli.dev/guide/animations#slide-transitions
transition: slide-left
# enable MDC Syntax: https://sli.dev/guide/syntax#mdc-syntax
mdc: true
---

# Memory Safety

Presented by Ray

<!--
The last comment block of each slide will be treated as slide notes. It will be visible and editable in Presenter Mode along with the slide. [Read more in the docs](https://sli.dev/guide/syntax.html#notes)
-->

---
transition: fade-out
layout: image-right
image: https://i.imgflip.com/8k3swd.jpg
---

# TLDR

Joe Biden says you should rewrite your c/c++ code in Rust.

<v-click>
Just kidding, in this presentation, I will be talking about memory safety and why it is important.

The motivation is an article published last month by the white house.

The article called <span class="underline text-blue-400">Press Release: Future Software Should Be Memory Safe</span>.

</v-click>

---
transition: slide-up
---

# What will be covered?

1. What is Memory Safety?
2. Why is it important?
3. How to achieve it?

---

# What is memory safety?

Memory safety is a property of some programming languages that prevents programmers from introducing certain types of bugs related to how memory is used. Since memory safety bugs are often security issues, memory safe languages are more secure than languages that are not memory safe.

Memory safe languages include Rust, Go, C#, Java, Swift, Python, and JavaScript. 

Languages that are not memory safe include C, C++, and assembly.

<div v-click class="mt-20">
  <hr/>
The problems that memory safety prevents include:
<span class="text-red-300">

1. Buffer overflows
1. Use-after-free errors
1. Memory leaks

</span>
</div>

<!--
You can have `style` tag in markdown to override the style for the current page.
Learn more: https://sli.dev/guide/syntax#embedded-styles
-->

<style>
h1 {
  background-color: #2B90B6;
  background-image: linear-gradient(45deg, #4EC5D4 10%, #146b8c 20%);
  background-size: 100%;
  -webkit-background-clip: text;
  -moz-background-clip: text;
  -webkit-text-fill-color: transparent;
  -moz-text-fill-color: transparent;
}
</style>

---
layout: two-cols
layoutClass: gap-16 flex items-center
---

## Buffer Overflow

```c{|4-5|6-7|10}
#include <stdio.h>

int main() {
  // buffer of size 10
  char buffer[10];
  // read input from user
  gets(buffer);
  // if the input is longer than 10 characters
  // it will overflow the buffer
  printf("%s\n", buffer);
  return 0;
}
```

<v-click>

</v-click>

::right::

<span class="text-gray-400"> 
  If we have a to do list with ten items, and we ask for the eleventh item, what should happen? Clearly we should receive an error of some sort. We should also get an error if we ask for the negative first item.

However in c/c++ the program will compile and run without any errors.

</span>

---
layout: two-cols
layoutClass: gap-16 flex items-center 
---

## Use-after-free

```c{|5-6|7-8|9-10}
#include <stdio.h>
#include <stdlib.h>

int main() {
  // allocate memory
  int* ptr = (int*)malloc(sizeof(int));
  // free the memory
  free(ptr);
  // use the memory
  *ptr = 10;
  return 0;
}
```


::right::
<span class="text-gray-400"> 
Imagine we delete a to do list and then later request the first item of that list. Clearly we should receive an error, as we shouldn't be able to get items from a deleted list. Languages that are not memory safe allow programs to fetch memory that they've said they are done with, and that may now be used for something else. The location in memory may now contain someone else's to do list! This is called a use-after-free vulnerability.
</span>

---
layout: two-cols
layoutClass: gap-16 flex items-center 
transition: slide-up
---

## Memory Leak

```c{|5-6|7-8}
#include <stdio.h>
#include <stdlib.h>

int main() {
  // allocate memory
  int* ptr = (int*)malloc(sizeof(int));
  // forget to free the memory
  return 0;
}
```

::right::
<span class="text-gray-400">
Imagine we have a to do list that we never delete. This is a memory leak. Memory leaks are a problem because they can cause the program to run out of memory. This can cause the program to crash, or it can cause the program to run very slowly.
</span>

---
layout: image-right
image: https://cover.sli.dev
---

# Why is memory safety important?

**Security**: Some of the most infamous cyber events in history – the Morris worm of 1988, the Slammer worm of 2003, the Heartbleed vulnerability in 2014, the Trident exploit of 2016, the Blastpass exploit of 2023 – were headline-grabbing cyberattacks that caused real-world damage to the systems that society relies on every day.

---
layout: image-left
image: https://cover.sli.dev
---

# How to achieve memory safety?

<div v-click>

1. Write safe code ( skill issue? )
</div>

<div v-click>

2. Use a memory safe language, such as Rust, Go etc.
</div>

---

# References

1. [Press Release: Future Software Should Be Memory Safe](https://www.whitehouse.gov/oncd/briefing-room/2024/02/26/press-release-technical-report/)
2. [What is memory safety and why does it matter?](https://www.memorysafety.org/docs/memory-safety/)

