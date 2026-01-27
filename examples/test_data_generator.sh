#!/bin/bash

set -e

echo "Generating additional test data files..."

cat > products.csv << 'EOF'
ProductID,ProductName,Category,Price,Stock
1,Laptop,Electronics,1200,50
2,Mouse,Electronics,25,200
3,Keyboard,Electronics,75,150
4,Monitor,Electronics,300,80
5,Desk,Furniture,450,30
6,Chair,Furniture,250,40
7,Lamp,Furniture,60,100
8,Notebook,Stationery,5,500
9,Pen,Stationery,2,1000
10,Pencil,Stationery,1,1200
EOF

cat > orders.csv << 'EOF'
OrderID,ProductID,Quantity,OrderDate,CustomerID
1001,1,2,2024-01-15,C001
1002,3,5,2024-01-16,C002
1003,5,1,2024-01-17,C001
1004,2,10,2024-01-18,C003
1005,8,50,2024-01-19,C002
1006,4,3,2024-01-20,C004
1007,7,2,2024-01-21,C001
1008,9,100,2024-01-22,C005
EOF

cat > customers.csv << 'EOF'
CustomerID,CustomerName,Email,Country
C001,John Smith,john@example.com,USA
C002,Jane Doe,jane@example.com,UK
C003,Bob Johnson,bob@example.com,Canada
C004,Alice Brown,alice@example.com,Australia
C005,Charlie Wilson,charlie@example.com,USA
EOF

cat > timeseries.csv << 'EOF'
Date,Value,Category
2024-01-01,100,A
2024-01-02,105,A
2024-01-03,98,A
2024-01-04,110,A
2024-01-05,115,A
2024-01-06,120,A
2024-01-07,118,A
2024-01-01,200,B
2024-01-02,210,B
2024-01-03,205,B
2024-01-04,220,B
2024-01-05,225,B
2024-01-06,230,B
2024-01-07,228,B
EOF

cat > missing_data.csv << 'EOF'
ID,Name,Age,Salary,Department
1,Alice,30,50000,Engineering
2,Bob,,60000,Sales
3,Charlie,35,,Marketing
4,David,40,70000,
5,Eve,,,Engineering
6,Frank,45,80000,Sales
EOF

cat > text_data.csv << 'EOF'
ID,Title,Description,Tags
1,Product Launch,Announcing our new product line for 2024,product;launch;2024
2,Quarterly Report,Financial results for Q1 2024,finance;report;q1
3,Team Meeting,Weekly sync-up with engineering team,meeting;engineering;weekly
4,Customer Feedback,Survey results from customer satisfaction,feedback;survey;customer
5,Marketing Campaign,New digital marketing strategy,marketing;digital;strategy
EOF

cat > coordinates.csv << 'EOF'
LocationID,Name,Latitude,Longitude
1,New York,40.7128,-74.0060
2,Los Angeles,34.0522,-118.2437
3,Chicago,41.8781,-87.6298
4,Houston,29.7604,-95.3698
5,Phoenix,33.4484,-112.0740
EOF

cat > anomaly_data.csv << 'EOF'
ID,Value,Timestamp
1,100,2024-01-01 10:00:00
2,105,2024-01-01 11:00:00
3,102,2024-01-01 12:00:00
4,500,2024-01-01 13:00:00
5,98,2024-01-01 14:00:00
6,103,2024-01-01 15:00:00
7,101,2024-01-01 16:00:00
8,1000,2024-01-01 17:00:00
9,99,2024-01-01 18:00:00
10,104,2024-01-01 19:00:00
EOF

cat > multi_type.csv << 'EOF'
Integer,Float,String,Boolean,Date
1,3.14,hello,true,2024-01-01
2,2.71,world,false,2024-01-02
3,1.41,test,true,2024-01-03
4,1.73,data,false,2024-01-04
5,2.23,cell,true,2024-01-05
EOF

cat > large_numbers.csv << 'EOF'
ID,Revenue,Expenses,Profit
1,1000000,750000,250000
2,2500000,1800000,700000
3,3200000,2400000,800000
4,1800000,1200000,600000
5,4500000,3000000,1500000
EOF

cat > dates_various.csv << 'EOF'
ID,DateISO,DateUS,DateEU
1,2024-01-15,01/15/2024,15/01/2024
2,2024-02-20,02/20/2024,20/02/2024
3,2024-03-25,03/25/2024,25/03/2024
4,2024-04-30,04/30/2024,30/04/2024
5,2024-05-10,05/10/2024,10/05/2024
EOF

cat > regex_test.csv << 'EOF'
ID,Email,Phone,Code
1,alice@example.com,555-1234,ABC-001
2,bob@test.org,555-5678,DEF-002
3,charlie@demo.net,555-9012,GHI-003
4,david@sample.com,555-3456,JKL-004
5,eve@example.org,555-7890,MNO-005
EOF

cat > correlation_data.csv << 'EOF'
Height,Weight,Age,Income
170,70,30,50000
175,75,35,60000
180,80,40,70000
165,65,25,45000
185,85,45,80000
160,60,22,40000
190,90,50,90000
EOF

cat > pivot_data.csv << 'EOF'
Region,Product,Quarter,Sales
North,Laptop,Q1,10000
North,Laptop,Q2,12000
North,Mouse,Q1,5000
North,Mouse,Q2,6000
South,Laptop,Q1,8000
South,Laptop,Q2,9000
South,Mouse,Q1,4000
South,Mouse,Q2,4500
East,Laptop,Q1,11000
East,Laptop,Q2,13000
West,Mouse,Q1,5500
West,Mouse,Q2,6500
EOF

echo "✓ Generated 15 additional test data files"
echo ""
echo "Files created:"
echo "  - products.csv (product catalog)"
echo "  - orders.csv (order transactions)"
echo "  - customers.csv (customer data)"
echo "  - timeseries.csv (time series data)"
echo "  - missing_data.csv (data with missing values)"
echo "  - text_data.csv (text analysis data)"
echo "  - coordinates.csv (geospatial data)"
echo "  - anomaly_data.csv (anomaly detection data)"
echo "  - multi_type.csv (multiple data types)"
echo "  - large_numbers.csv (large numeric values)"
echo "  - dates_various.csv (various date formats)"
echo "  - regex_test.csv (regex pattern testing)"
echo "  - correlation_data.csv (correlation analysis)"
echo "  - pivot_data.csv (pivot table data)"
