#!/usr/bin/Rscript
library(ggplot2)
t <- read.table('values.dat', header=TRUE)
t <- ggplot(t, aes(n, comparisons, colour = algorithm)) + geom_point() + geom_line() +scale_y_log10()
show(t)
browseURL("Rplots.pdf")