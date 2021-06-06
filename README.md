# pbni-rs
[![github](https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/gaoqiangz/pbni-rs)
[![crates.io](https://meritbadge.herokuapp.com/pbni-rs)](https://crates.io/crates/pbni-rs)
[![docs.rs](https://docs.rs/pbni-rs/badge.svg)](https://docs.rs/pbni-rs)
![BSD-2-Clause licensed](https://img.shields.io/crates/l/actix-web.svg)

pbni-rs是[`PBNI`]的Rust绑定,使开发者可以使用Rust语言进行PowerBuilder扩展开发.<br>
**[注意]** pbni-rs只支持PowerBuilder 10及以上版本.

# Feature flags

| Flag              | Description                                              | Default    |
|-------------------|----------------------------------------------------------|------------|
| `global_function` | 全局函数导出                                              | `enabled`  |
| `nonvisualobject` | 不可视对象导出                                            | `enabled`  |
| `visualobject`    | 可视对象导出                                              | `enabled`  |
| `decimal`         | 日期类型处理,将引入[`rust_decimal`]库                      | `enabled`  |
| `datetime`        | 日期类型处理,将引入[`chrono`]库                            | `enabled`  |
| `vm`              | 加载虚拟机以及创建[`Session`]等功能,将引入[`libloading`]库  | `disabled`  |

[`rust_decimal`]: https://crates.io/crates/rust_decimal
[`chrono`]: https://crates.io/crates/chrono
[`libloading`]: https://crates.io/crates/libloading

# 什么是PBNI?

[`PBNI`]是PowerBuilder虚拟机的C++扩展接口(PowerBuilder Native Interface).

![Figure]

通过[`PBNI`]接口我们可以使用底层语言与PBVM进行集成交互,极大的扩展了PowerBuilder的能力.

其他托管语言类似的技术有[`JNI`],[`C++/CLI`]等.

[`PBNI`]: https://docs.appeon.com/pb2019/native_interface_programmers_guide_and_reference/ch01s01.html
[`JNI`]: https://docs.oracle.com/javase/8/docs/technotes/guides/jni/spec/intro.html
[`C++/CLI`]: https://docs.microsoft.com/en-us/cpp/dotnet/native-and-dotnet-interoperability?view=msvc-160
[Figure]: http://infocenter.sybase.com/help/topic/com.sybase.infocenter.dc37794.1250/html/pbnigref/pbni03.gif

# 开始使用

添加`pbni-rs`到`Cargo.toml`即可使用:

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
pbni-rs = "0.1.0"
```

# 数据类型映射

|PowerBuilder|Rust|
|---|---|
|`int`|`pbint`,`i16`|
|`uint`|`pbuint`,`u16`|
|`long`|`pblong`,`i32`|
|`ulong`|`pbulong`,`u32`|
|`longlong`|`pblonglong`,`i64`|
|`real`|`pbreal`,`f32`|
|`double`|`pbdouble`,`f64`|
|`decimal`|[`Decimal`] (需要开启`decimal`特性)|
|`byte`|`pbbyte`,`u8`|
|`boolean`|`bool`|
|`char`|`PBChar`|
|`string`|`&PBStr`,`PBString`,`String`|
|`blob`|`&[u8]`,`Vec<u8>`|
|`date`|[`NaiveDate`] (需要开启`datetime`特性)|
|`time`|[`NaiveTime`] (需要开启`datetime`特性)|
|`datetime`|[`NaiveDateTime`] (需要开启`datetime`特性)|
|`any`|`Value`|
|任意对象|`Object`|
|任意数组|`Array`|

> PowerBuilder的所有类型都是Nullable的,Rust里使用`Option<T>`表示.<br>

# 字符串

PowerBuilder字符编码是[UTF-16LE],而Rust字符串编码采用的是[UTF-8]编码,这使得字符串操作时可能会有一点的性能损失.如果对性能有较高要求,请使用`&PBStr`进行交互,避免发生内存拷贝和编码转换.

pbni-rs提供了[`pbstr!`]宏在编译时生成`&'static PBStr`:

```rust
let rstr: &'static str = "hell world!";
let pstr: &'static PBStr = pbstr!("hell world!");
```

> pbni-rs使用[`widestring`]进行UTF-16编码转换.

[UTF-16LE]: https://en.wikipedia.org/wiki/UTF-16
[UTF-8]: https://en.wikipedia.org/wiki/UTF-8
[`widestring`]: https://crates.io/crates/widestring

# 内存安全

pbni-rs的Safe代码提供100%类型和内存安全保证,对于无法提供100%的内存安全保证的接口都使用了`unsafe`标记.最常见的就是获取引用,比如`&PBStr`.

```rust
impl<'obj> Object<'obj> {
    pub unsafe fn get_var_str(&self, fid: impl VarId) -> Option<&'obj PBStr> { ... }
    pub fn get_var_string(&self, fid: impl VarId) -> Option<PBString> { ... }
    pub fn set_var_str(&mut self, fid: impl VarId, value: impl AsPBStr) -> Result<()> { ... }
}
```

可以看到`Object`的`get_var_str`是`unsafe`方法,而`get_var_string`则是Safe的,这是因为像`set_var_str`这样的方法可能会修改`get_var_str`返回引用的内存,导致垂悬引用([Dangling Reference]).<br>
pbni-rs无法避免这种情况,因为对象的内部状态不完全由Rust维护,有很多途径会导致内存被修改,所以pbni-rs中所有返回引用的方法都将是[Unsafe]的,需要开发者自己保证对其正确使用.

[Unsafe]: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
[Dangling Reference]: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#dangling-references

# 线程安全

`Session`及其所有分配的资源都不能跨线程访问(包括`Object`/`Array`),因此它们都不是`Send`和`Sync`的,跨线程访问建议结合消息队列实现.

# 代码生成

pbni-rs可以非常方便将Rust对象或函数与PowerBuilder建立映射,全部由pbni-rs生成代码,省去手写繁琐的样板代码的同时保证了类型安全.

#### 映射PowerBuilder全局函数

- PowerBuilder

```vbscript
global type gf_bit_or from function_object native "pbrs.dll"
end type

forward prototypes
global function long gf_bit_or (readonly long a,readonly long b)
end prototypes
```

- C++

```cpp
#include <pbext.h>

PBXRESULT bit_or(PBCallInfo *ci)
{
    pblong a = ci->pArgs->GetAt(0)->GetLong();
    pblong b = ci->pArgs->GetAt(1)->GetLong();
    return ci->returnValue->SetLong(a|b);
}
PBXRESULT PBXCALL PBX_InvokeGlobalFunction(
    IPB_Session *pbsession,
    LPCWSTR functionName,
    PBCallInfo *ci)
{
    if(::wcscmp(functionName,L"gf_bit_or") == 0)
        return bit_or(ci);
    return PBX_E_NO_REGISTER_FUNCTION;
}
```

- Rust(pbni-rs)

```rust
use pbni::*;

#[global_function(name="gf_bit_or")]
fn bit_or(a: pblong, b: pblong) -> pblong {
    a | b
}
```

#### 映射PowerBuilder对象

- PowerBuilder

```vbscript
forward
global type n_pbni from nonvisualobject
end type
end forward

global type n_pbni from nonvisualobject native "pbrs.dll"
public function string of_hello (string world)
end type
global n_pbni n_pbni

on n_pbni.create
call super::create
TriggerEvent( this, "constructor" )
end on

on n_pbni.destroy
TriggerEvent( this, "destructor" )
call super::destroy
end on
```

- C++

```cpp
#include <pbext.h>

class CppObject: public IPBX_NonVisualObject
{
    IPB_Session *session;
    pbobject ctx;

    PBXRESULT handle_hello(PBCallInfo *ci)
    {
        LPCWSTR lpcsWorld = this->session->GetString(ci->pArgs->GetAt(0)->GetString());
        std::wostringstream ss;
        ss << L"hello " << lpcsWorld << L"!";
        return ci->returnValue->SetString(ss.str().c_str());
    }

public:
    CppObject(IPB_Session *pbsession,pbobject pbobj)
    :session(pbsession),
    ctx(pbobj)
    {}
    virtual ~CppObject() override {};

    virtual void Destroy() override { delete this; }

    virtual PBXRESULT Invoke(
        IPB_Session *session,
        pbobject obj,
        pbmethodID mid,
        PBCallInfo *ci) override
   {
        if(mid == 0)
            return this->handle_hello(ci);
        return PBX_E_NO_REGISTER_FUNCTION;
   }
};

PBXRESULT PBXCALL PBX_CreateNonVisualObject(
    IPB_Session *pbsession,
    pbobject pbobj,
    LPCWSTR className,
    IPBX_NonVisualObject **obj)
{
    if(::wcscmp(className,L"n_pbni") == 0)
    {
        *obj = new CppObject(pbsession,pbobj);
        return PBX_OK;
    }
    return PBX_E_NO_SUCH_CLASS;
}
```

- Rust(pbni-rs)

```rust
use pbni::*;

struct RustObject {
    session: Session,
    ctx: ContextObject
}

#[nonvisualobject(name = "n_pbni")]
impl RustObject {
    #[constructor]
    fn new(session: Session, ctx: ContextObject) -> RustObject {
        RustObject {
            session,
            ctx
        }
    }
    #[method(name="of_Hello")]
    fn hello(&self, world: String) -> String {
        format!("hello {}!",world)
    }
}
```
#### 参数提取

pbni-rs代码生成宏会自动提取PB参数为Rust映射的[数据类型],参数的提取顺序与PB端定义的顺序保持一致.其中有几个特殊的参数: [`Session`]/[`CallInfoRef`]/[`ArgumentsRef`],这几个参数对位置没有要求并且数量任意.

[数据类型]: #数据类型映射

```rust
use pbni::*;

#[global_function(name="gf_bit_or")]
fn bit_or(session: Session,a: pblong, b: pblong) -> pblong {
    a | b
}

//等同于

#[global_function(name="gf_bit_or")]
fn bit_or(session: Session,args: ArgumentsRef,a: pblong) -> pblong {
    a | args.get(1).get_long().unwrap()
}
```

**注意** Rust端参数列表须与PB端定义的类型数量以及顺序一致,任何不匹配的情况都会在运行时触发异常. <br>
当参数列表通过`CallInfoRef`/`ArgumentsRef`接收后,将不再匹配参数数量,因为这两个参数已经隐式表示接收了所有的参数.`CallInfoRef`/`ArgumentsRef`一般用于处理引用传递参数以及变长参数列表.

#### 可选参数列表匹配

以下示例为重载可选参数列表的匹配映射

- PowerBuilder

```vbscript
global type gf_test from function_object native "pbrs.dll"
end type

forward prototypes
global function long gf_test (readonly long a,readonly long b)
global function long gf_test (readonly long a,readonly long b,readonly long c)
global function long gf_test (readonly long a,readonly long b,readonly long c,readonly long d)
end prototypes
```

- Rust(pbni-rs)

```rust
use pbni::*;

#[global_function(name="gf_test")]
fn test(a: pblong, b: pblong, c: Option<long>, d: Option<long>) -> pblong {
    a | b
}
```