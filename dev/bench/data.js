window.BENCHMARK_DATA = {
  "lastUpdate": 1679681053049,
  "repoUrl": "https://github.com/purplesmoke05/web3-rush.py",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "15183665+purplesmoke05@users.noreply.github.com",
            "name": "Yosuke Otosu",
            "username": "purplesmoke05"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5f86f36910dd18f73f7f1361f226caea0c5344cc",
          "message": "Merge pull request #2 from purplesmoke05/dev\n\nimplement eth method",
          "timestamp": "2023-03-25T02:58:38+09:00",
          "tree_id": "b5564530af4c823fc47f57c3dda3c21abd5ae14b",
          "url": "https://github.com/purplesmoke05/web3-rush.py/commit/5f86f36910dd18f73f7f1361f226caea0c5344cc"
        },
        "date": 1679681052594,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark/test_bench_eth.py::TestBenchEthBlockNumber::test_block_number[web3]",
            "value": 1595.823324382706,
            "unit": "iter/sec",
            "range": "stddev: 0.0000874161139662466",
            "extra": "mean: 626.6357839999728 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestBenchEthBlockNumber::test_block_number[original]",
            "value": 714.9033995037661,
            "unit": "iter/sec",
            "range": "stddev: 0.00006691551273751631",
            "extra": "mean: 1.3987903830001756 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestBenchEthChainId::test_chain_id[web3]",
            "value": 1543.1822040479753,
            "unit": "iter/sec",
            "range": "stddev: 0.0000344852858117448",
            "extra": "mean: 648.0116199998065 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestBenchEthChainId::test_chain_id[original]",
            "value": 740.5565835179592,
            "unit": "iter/sec",
            "range": "stddev: 0.00006794556501054577",
            "extra": "mean: 1.350335710000138 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestBenchEthSyncing::test_syncing[web3]",
            "value": 1506.0951505072092,
            "unit": "iter/sec",
            "range": "stddev: 0.00010895510884651973",
            "extra": "mean: 663.968673999932 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestBenchEthSyncing::test_syncing[original]",
            "value": 710.1085877601332,
            "unit": "iter/sec",
            "range": "stddev: 0.00017866431120010392",
            "extra": "mean: 1.4082353279999893 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestBenchEthAccounts::test_accounts[web3]",
            "value": 1241.013603321055,
            "unit": "iter/sec",
            "range": "stddev: 0.00005458580510672625",
            "extra": "mean: 805.7929399999466 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestBenchEthAccounts::test_accounts[original]",
            "value": 420.2981629295408,
            "unit": "iter/sec",
            "range": "stddev: 0.00014045192927622494",
            "extra": "mean: 2.3792633140003545 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestBenchEthSendTransaction::test_send_transaction[web3]",
            "value": 147.02480246577653,
            "unit": "iter/sec",
            "range": "stddev: 0.0026817453439785676",
            "extra": "mean: 6.801573498000607 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestBenchEthSendTransaction::test_send_transaction[original]",
            "value": 55.10062712438749,
            "unit": "iter/sec",
            "range": "stddev: 0.0010115954669341247",
            "extra": "mean: 18.148613767000143 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestBenchEthWaitForTransactionReceipt::test_wait_for_transaction_receipt[web3]",
            "value": 1233.4663107461274,
            "unit": "iter/sec",
            "range": "stddev: 0.0010266759761483112",
            "extra": "mean: 810.7233989999258 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestBenchEthWaitForTransactionReceipt::test_wait_for_transaction_receipt[original]",
            "value": 402.2157125637707,
            "unit": "iter/sec",
            "range": "stddev: 0.001043221333191687",
            "extra": "mean: 2.486228083000242 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestBenchEthGetTransaction::test_get_transaction[web3]",
            "value": 923.8507031876115,
            "unit": "iter/sec",
            "range": "stddev: 0.00009922211843176631",
            "extra": "mean: 1.082425977000014 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestBenchEthGetTransaction::test_get_transaction[original]",
            "value": 356.8187659215867,
            "unit": "iter/sec",
            "range": "stddev: 0.0001099314478655909",
            "extra": "mean: 2.8025431829999548 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestBenchEthGetTransactionCount::test_get_transaction_count[web3]",
            "value": 1447.7232582133772,
            "unit": "iter/sec",
            "range": "stddev: 0.0000576680105415468",
            "extra": "mean: 690.7397490001586 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestBenchEthGetTransactionCount::test_get_transaction_count[original]",
            "value": 443.7847481600074,
            "unit": "iter/sec",
            "range": "stddev: 0.00012144170897529548",
            "extra": "mean: 2.2533446769996885 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestBenchEthGetBlock::test_get_block[web3]",
            "value": 1238.1158276079475,
            "unit": "iter/sec",
            "range": "stddev: 0.000032771741075337654",
            "extra": "mean: 807.6788759998408 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestBenchEthGetBlock::test_get_block[original]",
            "value": 511.9586876043449,
            "unit": "iter/sec",
            "range": "stddev: 0.00005488091132211899",
            "extra": "mean: 1.9532826069997784 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestBenchEthGetBalance::test_get_balance[web3]",
            "value": 1594.8717473124727,
            "unit": "iter/sec",
            "range": "stddev: 0.000015864349661031946",
            "extra": "mean: 627.0096649997754 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestBenchEthGetBalance::test_get_balance[original]",
            "value": 447.56307208637,
            "unit": "iter/sec",
            "range": "stddev: 0.00009176575946331134",
            "extra": "mean: 2.2343219589998284 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestBenchEthGetCode::test_get_code[web3]",
            "value": 1470.1976772810635,
            "unit": "iter/sec",
            "range": "stddev: 0.00010110645801548847",
            "extra": "mean: 680.1806420000389 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestBenchEthGetCode::test_get_code[original]",
            "value": 446.8687039858043,
            "unit": "iter/sec",
            "range": "stddev: 0.00003544738059505743",
            "extra": "mean: 2.237793766000152 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestBenchEthEstimateGas::test_estimate_gas[web3]",
            "value": 1373.9318639787114,
            "unit": "iter/sec",
            "range": "stddev: 0.00003192129568195684",
            "extra": "mean: 727.8381310002828 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestBenchEthEstimateGas::test_estimate_gas[original]",
            "value": 231.32827431829895,
            "unit": "iter/sec",
            "range": "stddev: 0.0000875890343133436",
            "extra": "mean: 4.322861106999994 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestBenchEthGetLogs::test_get_logs[web3]",
            "value": 771.026501815592,
            "unit": "iter/sec",
            "range": "stddev: 0.00007994802914231373",
            "extra": "mean: 1.2969722799997498 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestBenchEthGetLogs::test_get_logs[original]",
            "value": 19.379562270889487,
            "unit": "iter/sec",
            "range": "stddev: 0.00031004732208559575",
            "extra": "mean: 51.600752690999855 msec\nrounds: 100"
          }
        ]
      }
    ]
  }
}