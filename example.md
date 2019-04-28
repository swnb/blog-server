# 2019 年 4 月 22 日

## 从现实中理解 monad

> 笔者不是很熟悉 haskell ，错误的地方欢迎指正。

### 纯函数

haskell 的函数是无副作用的，解释起来就是：同一个函数在输入相同的情况下输出必须相同。并且任何地方，任何时间调用都不会对外界产生影响。也就是说，纯函数是**正确**的，而由纯函数**组合**后的函数也是**正确**，这也就是 haskell 号称的无运行时崩溃的原因。因为它是**正确**的。

下面通过`ts`和`rust`说说什么是 monad。

> 代码的作用是读一个名为`example`的文件内容，然后将内容解析成数字;

理想的状态下代码是这样的：

```typescript
// typescript

import fs from 'fs';

const f1 = (): string => fs.readFileSync('./example').toString();

const f2 = (s: string): number => parseInt(s, 10);

const result = f2(f1());

console.log(result);
```

```rust
// rust

fn f1() -> String {
	std::fs::read_to_string("./example").unwrap()
}

fn f2(s: String) -> i32 {
	s.parse().unwrap()
}

fn main() {
	let result = f2(f1());
	println!("{}", result);
}
```

上面的例子中，f1 和 f2 通过组合读取文件然后解析成数字。其中 f1 无输入，输出字符，f2 输入字符串，输出数字。

但是真实世界里面，上面的函数 f1, f2 都是不纯的，输出是不确定。

1. 读取文件可能出错。
2. 解析字符串成为数字，js 有可能输出 NaN, rust 有可能直接崩溃退出。

所以真实世界并不是纯的。monad 就是为了解决这个问题而诞生的。

---

为了解决这个问题，首先定义一个类型 M 将数据 T 包裹起来。

```typescript
// typescript

type M<T> = T | Error;
```

```rust
// rust

enum M<T> {
	Ok(T),
	Error,
}
```

那么 f1,f2 的代码经过变换变成了

```typescript
// typescript

import fs from 'fs';

const f1 = (): M<string> => {
	try {
		let str = fs.readFileSync('./example').toString();
		return str;
	} catch {
		return new Error("can't read file");
	}
};

const f2 = (s: string): M<number> => {
	const result = parseInt(s, 10);
	if (Number.isNaN(result)) {
		return new Error('parse result is NaN');
	} else {
		return result;
	}
};
```

```rust
// rust

fn f1() -> M<String> {
	match std::fs::read_to_string("./example") {
		Ok(s) => M::Ok(s),
		Err(_) => M::Error,
	}
}

fn f2(s: String) -> M<i32> {
	match s.parse() {
		Ok(n) => M::Ok(n),
		Err(_) => M::Error,
	}
}
```

下面要进行 f1 和 f2 的组合，我们需要定义一个函数，只要实现了这个函数，M 类型就可以被称为 monad。

> 建议仔细的看看这个函数的定义。第一个参数是 `M<T>` 类型，第二个参数是一个函数，这个函数接受 `T` 类型的数据返回 `M<U>` 类型,整个函数最终返回 `M<U>` 类型的数据。

```
>>= : (M<T>, T -> M<U>) -> M<U>
```

这个函数的实现：

```typescript
// typescript

const bind = <T, U>(result1: M<T>, f2: (arg: T) => M<U>): M<U> => {
	if (result1 instanceof Error) {
		return result1;
	} else {
		return f2(result1);
	}
};
```

```rust
// rust

fn bind<T, U>(result1: M<T>, f2: impl Fn(T) -> M<U>) -> M<U> {
	match result1 {
		M::Ok(s) => f2(s),
		_ => M::Error,
	}
}
```

这样就可以拿来拼接函数了,最终的结果如下

```typescript
// typescript

import fs from 'fs';

type M<T> = T | Error;

const f1 = (): M<string> => {
	try {
		let str = fs.readFileSync('./example').toString();
		return str;
	} catch {
		return new Error("can't read file");
	}
};

const f2 = (s: string): M<number> => {
	const result = parseInt(s, 10);
	if (Number.isNaN(result)) {
		return new Error('parse result is NaN');
	} else {
		return result;
	}
};

const bind = <T, U>(result1: M<T>, f2: (arg: T) => M<U>): M<U> => {
	if (result1 instanceof Error) {
		return result1;
	} else {
		return f2(result1);
	}
};

bind(f1(), f2);
```

```rust
// rust

enum M<T> {
	Ok(T),
	Error,
}

fn f1() -> M<String> {
	match std::fs::read_to_string("./example") {
		Ok(s) => M::Ok(s),
		Err(_) => M::Error,
	}
}

fn f2(s: String) -> M<i32> {
	match s.parse() {
		Ok(n) => M::Ok(n),
		Err(_) => M::Error,
	}
}

fn bind<T, U>(result1: M<T>, f2: impl Fn(T) -> M<U>) -> M<U> {
	match result1 {
		M::Ok(s) => f2(s),
		_ => M::Error,
	}
}

fn main() {
	bind(f1(), f2);
}
```

假设这样的函数非常多，你的代码就可以写成这样，向管道一样把函数串联起来。

```rust
bind(bind(bind(bind(f1(),f2),f3),f4),f5);
```

通过上面的例子可以看出 `future/promise` 就是 `monad` ，为了说明这个，我把 ts 的部分改成 Promise 版本。

```typescript
// Promise

import fs, { read } from 'fs';
import util from 'util';

type M<T> = Promise<T>;
const bind = Promise.prototype.then;

const readFile = util.promisify(fs.readFile);
const f1 = (): M<string> => readFile('./example').then(data => data.toString());

const f2 = (s: string): M<number> =>
	new Promise((res, rej) => {
	    const result = parseInt(s, 10);
	 	if (Number.isNaN(result)) {
			rej("can't parse string");
		} else {
           res(result);
		}
	});

bind.call(f1(), f2).catch(console.error);
```

我将 Promise 换成了 M ,then 换成了 bind，可以看出 Promise 就是 monad；

### 总结

可以看到 monad 属于一种编写代码的方式，目的是为了**处理**不纯函数的副作用。

个人理解的 monad，可能不太准确，错误地方欢迎指出。
