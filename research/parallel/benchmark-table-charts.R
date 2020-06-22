library(tidyverse)

benchmark_results <- readr::read_csv("benchmark-data/benchmark-table.csv")

sequential <- benchmark_results %>%
  filter(case == "Sequential validation")

parallel <- benchmark_results %>%
  filter(case == "Parallel validation")

scatter <- 
  ggplot(benchmark_results, aes(x = size, y = duration_micros)) +
    geom_jitter(aes(col = case))

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

benchmark_results %>% summary()
sequential %>% summary()
parallel %>%summary()


# Spread

sequential_narrow <- 
  sequential %>% 
  transmute(
    size = size,
    duration_micros_seq = duration_micros
  )

parallel_narrow <-
  parallel %>% 
  transmute(
    size = size,
    duration_micros_par = duration_micros
  )

comparative_table <- sequential_narrow %>% left_join(parallel_narrow, by=c("size"))

spread <-
  comparative_table %>%
  transmute(
    size = size,
    spread = duration_micros_par - duration_micros_seq
  )

ggplot(spread, aes(x=spread)) +
  geom_histogram(binwidth = 1000, color="black", fill = "#f59aa9")


summary(spread)
