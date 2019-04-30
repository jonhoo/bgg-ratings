#!/usr/bin/Rscript
d <- read.csv("ratings.csv", header = FALSE,
		col.names = c("rating", "ratings", "username"),
		colClasses = c("numeric", "integer", "character")
)
d <- d[,c("rating", "ratings")]
d

library(ggplot2)

d$group = "1"
d[d$ratings <= 5 & d$ratings > 1,]$group = "2-5"
d[d$ratings <= 10 & d$ratings > 5,]$group = "5-10"
d[d$ratings <= 25 & d$ratings > 10,]$group = "10-25"
d[d$ratings <= 100 & d$ratings > 25,]$group = "25-100"
d[d$ratings > 100,]$group = ">100"
d$group = factor(d$group, levels = c("1", "2-5", "5-10", "10-25", "25-100", ">100"))

g <- ggplot(d, aes(rating, color=group))
g <- g + labs(color = "# ratings")
g <- g + stat_ecdf(geom = "step")
g <- g + facet_wrap(. ~ group)
g <- g + xlab("Rating value")
g <- g + ylab("CDF")
g <- g + scale_x_continuous(breaks = seq(1, 10))
#g <- g + ylim(0, 11)
#g <- g + xlim(0, NA)
#g <- g + scale_x_continuous(trans='log10')
ggsave("corr.png", plot=g, width=6, height=4)

d$rating = round(d$rating)
d[d$ratings > 50,]$ratings = 50
g <- ggplot(d, aes(ratings, color=group))
g <- g + stat_ecdf(geom = "step")
g <- g + facet_wrap(. ~ rating)
g <- g + xlab("Ratings value")
g <- g + ylab("CDF")
#g <- g + ylim(0, 11)
#g <- g + xlim(0, NA)
#g <- g + scale_x_continuous(trans='log10')
ggsave("ocorr.png", plot=g, width=6, height=4)
