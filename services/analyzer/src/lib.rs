// should return a random statement of analysis
use rand::Rng;
pub fn get_analysis() -> String {
    let analysis = [
        "In the intersection of technology and humanity, we find the blueprint of our future.",
        "The patterns we uncover are not just reflections of the past, but whispers of what is yet to come.",
        "Data doesn't just reveal the truth; it constructs the narrative of our existence.",
        "In every anomaly lies the seed of a revolution, waiting to be understood.",
        "The silence of the data is often more telling than the noise it produces.",
        "Every trend is a testament to the unspoken agreements within a society.",
        "Numbers alone are inert; it's the human interpretation that breathes life into them.",
        "Our greatest insights are often found in the shadows of our initial assumptions.",
        "What we choose to measure defines what we consider to be valuable.",
        "The truth in data is less about precision and more about perspective.",
        "To predict the future, we must first understand the patterns of the present.",
        "Every outlier is a doorway to deeper understanding.",
        "Complexity in data often mirrors the complexity of the human experience.",
        "The most powerful insights are often those that challenge our preconceived notions.",
        "In the realm of data, certainty is an illusion, and probability is king.",
        "What is not measured often holds as much significance as what is measured.",
        "The story of a dataset is not in its numbers, but in the connections between them.",
        "True understanding lies in the synthesis of disparate data points.",
        "Data analysis is less about finding the right answer and more about asking the right questions.",
        "The greatest revelations come from the convergence of seemingly unrelated data streams."
    ];

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..analysis.len());
    analysis[random_index].to_string()
}
