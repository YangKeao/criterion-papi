use criterion::measurement::ValueFormatter;
use criterion::Throughput;

pub(crate) struct InsFormatter;

impl ValueFormatter for InsFormatter {
    fn scale_values(&self, _: f64, _: &mut [f64]) -> &'static str {
        "ins"
    }

    fn scale_throughputs(
        &self,
        _: f64,
        _: &Throughput,
        _: &mut [f64],
    ) -> &'static str {
        "ins"
    }

    fn scale_for_machines(&self, _: &mut [f64]) -> &'static str {
        "ins"
    }
}
