use newrelic::Segment;

pub fn end_transaction() {
    if *crate::ENABLE_NEW_RELIC {
        crate::TL_TRANSACTION.inner.with(|tr| {
            if let Some(ref mut val) = tr.borrow_mut().as_mut() {
                val.as_mut().and_then(|tr| Some(tr.end()));
            }
        });
    }
}

// TODO: Need to change it
#[allow(dead_code)]
fn start_custom_segment<'a>(name: &str) -> Segment<'a> {
    if *crate::ENABLE_NEW_RELIC {
        crate::TL_TRANSACTION.inner.with(|tr| {
            let tr = tr.borrow();
            match tr.as_ref() {
                Some(ref val) => {
                    match val.as_ref() {
                        Some(tr) => {
                            tr.create_custom_segment(name, "CUSTOM");
                        }
                        None => {
                            nullable_segment();
                        }
                    };
                }
                _ => unimplemented!(),
            };

            //            let t = if let Some(val) = tr.borrow().as_ref() {
            //                val.borrow().as_ref().map_or_else(
            //                    || Some(nullable_segment()),
            //                    |tr| Some(tr.create_custom_segment(name, "CUSTOM")),
            //                )
            //            } else {
            //                None
            //            };
            //            t
        });
    }
    return nullable_segment();
}

// pub fn end_custom_segment(segment)
#[allow(dead_code)]
fn nullable_segment() -> Segment<'static> {
    Segment::default()
}
