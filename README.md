# criterion-papi

```rs
let mut c = criterion::Criterion::default()
    .with_measurement(criterion_papi::PapiMeasurement::new("PAPI_TOT_INS"))
    .configure_from_args();
```

It accepts any PAPI event as the argument of `new`, then the counter was used as measurement.
