library(tidyverse)

benchmark_results <- readr::read_csv("benchmark-data/benchmark-table.csv")

scatter <- 
  ggplot(benchmark_results, aes(x = size, y = duration_micros)) +
    geom_point(aes(col = case))

box_plot <-
  ggplot(benchmark_results, aes(x = case, y = duration_micros)) +
    geom_boxplot() +
  stat_summary(
    fun.y = median,
    geom = "point",
    size = 3
  ) + 
  scale_y_continuous(trans='log10')

ggsave("benchmark-table-validation-scatter.png", scatter)
ggsave("benchmark-table-validation-boxplot.png", box_plot)
