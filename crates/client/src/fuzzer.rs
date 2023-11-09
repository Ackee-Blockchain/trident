use arbitrary::Arbitrary;
use arbitrary::Unstructured;

#[derive(Clone)]
pub struct FuzzData<T> {
    pub pre_ixs: Vec<T>,
    pub ixs: Vec<T>,
    pub post_ixs: Vec<T>,
}

#[allow(unused_variables)]
pub trait FuzzDataBuilder<T: for<'a> Arbitrary<'a>> {
    /// The instruction(s) executed as first, can be used for initialization.
    fn pre_ixs(u: &mut Unstructured) -> arbitrary::Result<Vec<T>> {
        Ok(vec![])
    }

    /// The main instructions for fuzzing.
    fn ixs(u: &mut Unstructured) -> arbitrary::Result<Vec<T>> {
        let v = <Vec<T>>::arbitrary(u)?;
        // Return always a vector with at least one element, othewise return error.
        if v.is_empty() {
            return Err(arbitrary::Error::NotEnoughData);
        }
        Ok(v)
    }

    /// The instuction(s) executed as last.
    fn post_ixs(u: &mut Unstructured) -> arbitrary::Result<Vec<T>> {
        Ok(vec![])
    }
}

#[macro_export]
macro_rules! fuzz_trd {
    ($ix:ident: $ix_dty:ident , |$buf:ident: $dty:ident| $body:block) => {
        fuzz(|$buf| {
            let $buf: FuzzData<$ix_dty> = {
                use arbitrary::Unstructured;

                let mut buf = Unstructured::new($buf);
                if let Ok(fuzz_data) = build_ix_fuzz_data($dty {}, &mut buf) {
                    fuzz_data
                } else {
                    return;
                }
            };
            Runtime::new().unwrap().block_on(async { $body });
        });
    };
}

pub fn build_ix_fuzz_data<U: for<'a> Arbitrary<'a>, T: FuzzDataBuilder<U>>(
    _data_builder: T,
    u: &mut arbitrary::Unstructured,
) -> arbitrary::Result<FuzzData<U>> {
    Ok(FuzzData {
        pre_ixs: T::pre_ixs(u)?,
        ixs: T::ixs(u)?,
        post_ixs: T::post_ixs(u)?,
    })
}
