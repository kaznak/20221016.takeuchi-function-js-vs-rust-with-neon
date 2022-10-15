use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn tarai_rs(x: f64, y: f64, z: f64) -> f64
{
    if x <= y {
        y
    } else {
        tarai_rs(tarai_rs(x-1.0,y,z),tarai_rs(y-1.0,z,x),tarai_rs(z-1.0,x,y))
    }
}

fn tarai(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let x = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let y = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let z = cx.argument::<JsNumber>(2)?.value(&mut cx);
    let fr = tarai_rs(x, y, z);
    let r: Handle<JsNumber> = cx.number(fr);

    Ok(r)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("tarai", tarai)?;
    Ok(())
}
