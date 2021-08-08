use ipn_sim_lib::report::Report;
use ipn_sim_lib::utils::TimeMetric;
use yew::Html;

pub trait GraphReport: Report {
    fn render_graph(&self, width: u16, height: u16, sim_time: TimeMetric) -> Html;
}