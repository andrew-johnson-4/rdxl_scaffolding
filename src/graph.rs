use rdxl::{xtype,xrender};

xtype!(
  /** BarGraph renders enclosed data as a bar graph */
  <!BarGraph xunit:String yunit:String><!BarGraphItem x:String y:u64/></BarGraph>
);
xtype!(
  /** Histogram renders enclosed data as a histogram */
  <!Histogram xunit:String yunit:String><!HistogramItem xmin:u64 xmax:u64 y:u64/></Histogram>
);
xtype!(
  /** PieChart renders enclosed data as a pie chart */
  <!PieChart><!PieChartItem xtag:String y:u64/></PieChart>
);
xtype!(
  /** LineGraph renders enclosed data as a line graph */
  <!LineGraph xunit:String yunit:String><!LineGraphLine f:String/></LineGraph>
);

xrender!(BarGraph, 
  {{ let div_id = crate::util::unique_identifier("d3_div_"); }}
  <div id={{ format!("{}", div_id) }}></div>
  <script>
  r#"var margin = {top: 20, right: 20, bottom: 60, left: 60},
         width = 960 - margin.left - margin.right,
         height = 500 - margin.top - margin.bottom;

     var x = d3.scaleBand()
               .range([0, width])
               .padding(0.1);
     var y = d3.scaleLinear()
               .range([height, 0]);

     var svg = d3.select("# {{ format!("'#{}'", div_id) }} r#").append('svg')
                 .attr('width', width + margin.left + margin.right)
                 .attr('height', height + margin.top + margin.bottom)
                 .append('g')
                 .attr('transform', 'translate(' + margin.left + ',' + margin.top + ')');

     var data = ["#
     {{ for bgi in self.children.iter() {{
       {{ let BarGraphChildren::BarGraphItem(bgi) = bgi; }}
       "['" {{ bgi.x }} "'," {{ bgi.y }} "],"
     }} }}
  r#"];

     x.domain(data.map(function(d) { return d[0]; }));
     y.domain([0, d3.max(data, function(d) { return d[1]; })]);

     svg.selectAll('.bar')
        .data(data)
        .enter().append('rect')
        .attr('class', 'bar')
        .attr('x', function(d) { return x(d[0]); })
        .attr('width', x.bandwidth())
        .attr('y', function(d) { return y(d[1]); })
        .attr('height', function(d) { return height - y(d[1]); });

     svg.append('g')
        .attr('transform', 'translate(0,' + height + ')')
        .call(d3.axisBottom(x));

     svg.append('g')
        .call(d3.axisLeft(y));

     svg.append('text')
        .attr('transform',
              'translate(' + (width/2) + ',' +
                             (height + margin.top + 20) + ')')
        .style('text-anchor', 'middle')
        .text("# {{ format!("'{}'", self.xunit) }} r#");

     svg.append('text')
        .attr('transform', 'rotate(-90)')
        .attr('y', 0 - margin.left)
        .attr('x',0 - (height / 2))
        .attr('dy', '1em')
        .style('text-anchor', 'middle')
        .text("# {{ format!("'{}'", self.yunit) }} r#");"#
  </script>
);

xrender!(Histogram, 
  {{ let div_id = crate::util::unique_identifier("d3_div_"); }}
  <div id={{ format!("{}", div_id) }}></div>
  <script>
  r#"var margin = {top: 20, right: 20, bottom: 60, left: 60},
         width = 960 - margin.left - margin.right,
         height = 500 - margin.top - margin.bottom;

     var x = d3.scaleBand()
               .range([0, width])
               .padding(0.1);
     var y = d3.scaleLinear()
               .range([height, 0]);

     var svg = d3.select("# {{ format!("'#{}'", div_id) }} r#").append('svg')
                 .attr('width', width + margin.left + margin.right)
                 .attr('height', height + margin.top + margin.bottom)
                 .append('g')
                 .attr('transform', 'translate(' + margin.left + ',' + margin.top + ')');

     var data = ["#
     {{ for hgi in self.children.iter() {{
       {{ let HistogramChildren::HistogramItem(hgi) = hgi; }}
       "[" {{ hgi.xmin }} "," {{ hgi.xmax }} "," {{ hgi.y }} "],"
     }} }}
  r#"];

     x.domain(data.map(function(d) { return d[0]; }));
     y.domain([0, d3.max(data, function(d) { return d[2]; })]);

     svg.selectAll('.bar')
        .data(data)
        .enter().append('rect')
        .attr('class', 'bar')
        .attr('x', function(d) { return x(d[0]); })
        .attr('width', x.bandwidth())
        .attr('y', function(d) { return y(d[2]); })
        .attr('height', function(d) { return height - y(d[2]); });

     svg.append('g')
        .attr('transform', 'translate(0,' + height + ')')
        .call(d3.axisBottom(x));

     svg.append('g')
        .call(d3.axisLeft(y));

     svg.append('text')
        .attr('transform',
              'translate(' + (width/2) + ',' +
                             (height + margin.top + 20) + ')')
        .style('text-anchor', 'middle')
        .text("# {{ format!("'{}'", self.xunit) }} r#");

     svg.append('text')
        .attr('transform', 'rotate(-90)')
        .attr('y', 0 - margin.left)
        .attr('x',0 - (height / 2))
        .attr('dy', '1em')
        .style('text-anchor', 'middle')
        .text("# {{ format!("'{}'", self.yunit) }} r#");"#
  </script>
);
