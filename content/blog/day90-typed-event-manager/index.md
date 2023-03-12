+++
title = "Day90 - Typed Event Manager"
description = "Strongly Typed Event Manager in Typescript"
date = 2019-10-23
+++

Recently I have been spending time with Typescript and found myself wishing there was an event manager which I could use
for my projects. Frustratingly all of the event managers I found (very possible I missed a better one) were too
complicated, or didn't have very ergonomic types. So I wrote my own and have been iterating on it for a while. At this
point I'm very happy with it as its super succinct and yet gets the job done.

# The Code

{% code(lang="typescript") %}
type Tuple = any[];

export class EventManager<TArgs extends Tuple = [], TResult = void> {
  currentId = 0;
  subscriptions: Map<number, (...args: TArgs) => TResult> = new Map();

  Subscribe(callback: (...args: TArgs) => TResult) {
    let id = this.currentId;
    this.subscriptions.set(id, callback);
    this.currentId++;
    return id;
  }

  Unsubscribe(id: number) {
    return this.subscriptions.delete(id);
  }

  Publish(...args: TArgs) {
    let results: TResult[] = [];
    for (let id of this.subscriptions.keys()) {
      results.push(this.subscriptions.get(id).apply(null, args));
    }
    return results;
  }
}
{% end %}

Just 25 lines long, and yet its doing some interesting things with generics to allow this single class to handle
arbitrary arguments and aggregated return values. I'll take it section by section. First though an example of the
usage.

{% code(lang="typescript") %}
let fizzbuzz = new EventManager<[number], string>();

fizzbuzz.Subscribe(i => i % 3 == 0 ? "Fizz" : "");
fizzbuzz.Subscribe(i => i % 5 == 0 ? "Buzz" : "");

for (let i = 0; i < 100; i++) {
  console.log(fizzbuzz.Publish(i).join());
}
{% end %}

Conveniently the type of the function passed to fizzbuzz.Subscribe is inferred as Subscribe takes a function with a
single number as the argument, and which returns a string. Similarly the result of publish is an array of strings
returned from all of the subscribers. The code is strongly typed, and clean.

## Generics

{% code(lang="typescript") %}
type Tuple = any[];

export class EventManager<TArgs extends Tuple = [], TResult = void> {
{% end %}

The contribution for this event manager is contained in these two lines. TArgs is a generic which must be any array.
This means it could either be a variable length array of some type, or a strongly typed Tuple with types for each index.
I use TArgs to type each argument of subscribers to this event manager. In the example above, the generic is the tuple
`[number]` which is a Tuple with a single number in it.

The class definition also has default values for both so that a simple `EventManager` with no arguments and no return
value is as simple as possible.

## State

{% code(lang="typescript") %}
currentId = 0;
subscriptions: Map<number, (...args: TArgs) => TResult> = new Map();
{% end %}

Here the state is defined for the EventManager class which is comprised of an identifier to track subscriptions, and a
subscriptions map which stores the callbacks themselves. The important bit here is that the subscription type uses args
expansion to specify the type of the arguments. In typescript if the type of the args getting spread is a strongly typed
Tuple, then the arguments are strongly typed as well. So the type:

{% code(lang="typescript") %}
type Callback = (...[number, string]) => void;
{% end %}

is equivalent to

{% code(lang="typescript") %}
type Callback = (firstArg: number, secondArg: string) => void;
{% end %}

Thats about all of the complicated type stuff in the event manager which enables the single class to handle any arity of
event.

## Subscribe/Unsubscribe

{% code(lang="typescript") %}
Subscribe(callback: (...args: TArgs) => TResult) {
  let id = this.currentId;
  this.subscriptions.set(id, callback);
  this.currentId++;
  return id;
}

Unsubscribe(id: number) {
  return this.subscriptions.delete(id);
}
{% end %}

These functions add and remove subscriptions to a given event manager. They are pretty straight forward and use the id
state to assign an unique field to each subscription. This unique id is then returned to the user and may be used to
remove the subscription in the future.

## Publish

{% code(lang="typescript") %}
Publish(...args: TArgs) {
  let results: TResult[] = [];
  for (let id of this.subscriptions.keys()) {
    results.push(this.subscriptions.get(id).apply(null, args));
  }
  return results;
}
{% end %}

Finally the publish function takes arguments with types taken from the TArgs generic tuple, and calls every subscription
passing the args list. The apply method on the subscription functions is used to allow passing an array of arguments as
though they were passed one by one. This keeps the typing sound and allows the publish function to just look like a
normal function with normal arguments.

The result of each subscription call is then stored in an array and returned to the user.

## Conclusion

This simple class has made a huge difference in my code and I use it constantly. The type system in Typescript is now flexible 
enough in typescript to enable a class like this while still keeping the code safe and consistent. The more I use it,
the more I appreciate the awesome work they are doing over on the Typescript team.

Till tomorrow,  
Kaylee
