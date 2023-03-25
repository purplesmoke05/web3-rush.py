window.BENCHMARK_DATA = {
  "lastUpdate": 1679761977159,
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
      },
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
          "id": "1a7014256c7104085029a03268de6891158f317a",
          "message": "add: net/geth module (#3)\n\n* up\r\n\r\n* up\r\n\r\n* update ci",
          "timestamp": "2023-03-26T01:26:58+09:00",
          "tree_id": "80e91408304cb5d3213438c2edca44a466f0fb91",
          "url": "https://github.com/purplesmoke05/web3-rush.py/commit/1a7014256c7104085029a03268de6891158f317a"
        },
        "date": 1679761976279,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthBlockNumber::test_block_number[rush]",
            "value": 1690.9979428581257,
            "unit": "iter/sec",
            "range": "stddev: 0.0000285401709984551",
            "extra": "mean: 591.3667750002105 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthBlockNumber::test_block_number[original]",
            "value": 753.8997844905225,
            "unit": "iter/sec",
            "range": "stddev: 0.000032460342384899266",
            "extra": "mean: 1.3264362460002417 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthChainId::test_chain_id[rush]",
            "value": 1663.0253534706796,
            "unit": "iter/sec",
            "range": "stddev: 0.000008517977109559511",
            "extra": "mean: 601.3137430003894 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthChainId::test_chain_id[original]",
            "value": 777.8336948592774,
            "unit": "iter/sec",
            "range": "stddev: 0.000056312424557103055",
            "extra": "mean: 1.2856218580000132 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSyncing::test_syncing[rush]",
            "value": 1691.9772504522027,
            "unit": "iter/sec",
            "range": "stddev: 0.000012550915799757602",
            "extra": "mean: 591.0244949999992 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSyncing::test_syncing[original]",
            "value": 788.1405055532917,
            "unit": "iter/sec",
            "range": "stddev: 0.00002750751664588489",
            "extra": "mean: 1.2688092959998016 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthAccounts::test_accounts[rush]",
            "value": 1296.6660247641703,
            "unit": "iter/sec",
            "range": "stddev: 0.00003084292706198305",
            "extra": "mean: 771.2086080005633 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthAccounts::test_accounts[original]",
            "value": 448.1461945527539,
            "unit": "iter/sec",
            "range": "stddev: 0.00004664625743358431",
            "extra": "mean: 2.231414685999937 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSendTransaction::test_send_transaction[rush]",
            "value": 128.57435116377476,
            "unit": "iter/sec",
            "range": "stddev: 0.003229394856845044",
            "extra": "mean: 7.777600983000298 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSendTransaction::test_send_transaction[original]",
            "value": 51.79693434903622,
            "unit": "iter/sec",
            "range": "stddev: 0.0010756377409315411",
            "extra": "mean: 19.30616189100016 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthWaitForTransactionReceipt::test_wait_for_transaction_receipt[rush]",
            "value": 1221.563923459611,
            "unit": "iter/sec",
            "range": "stddev: 0.0010204329877102542",
            "extra": "mean: 818.6227350001332 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthWaitForTransactionReceipt::test_wait_for_transaction_receipt[original]",
            "value": 419.16570957302315,
            "unit": "iter/sec",
            "range": "stddev: 0.0010352740369276175",
            "extra": "mean: 2.3856913320000217 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransaction::test_get_transaction[rush]",
            "value": 918.729902507583,
            "unit": "iter/sec",
            "range": "stddev: 0.000049660138020636",
            "extra": "mean: 1.0884591840002145 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransaction::test_get_transaction[original]",
            "value": 361.5660957319479,
            "unit": "iter/sec",
            "range": "stddev: 0.000049646687531984265",
            "extra": "mean: 2.765746047000391 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransactionCount::test_get_transaction_count[rush]",
            "value": 1561.7346475335369,
            "unit": "iter/sec",
            "range": "stddev: 0.00004131326342985214",
            "extra": "mean: 640.3136420001374 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransactionCount::test_get_transaction_count[original]",
            "value": 470.47986861436345,
            "unit": "iter/sec",
            "range": "stddev: 0.00012235201682610404",
            "extra": "mean: 2.125489455999798 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBlock::test_get_block[rush]",
            "value": 1246.2426283248606,
            "unit": "iter/sec",
            "range": "stddev: 0.000026538472245748253",
            "extra": "mean: 802.4119680002855 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBlock::test_get_block[original]",
            "value": 530.2214317953762,
            "unit": "iter/sec",
            "range": "stddev: 0.00002833318129340108",
            "extra": "mean: 1.8860044880002533 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBalance::test_get_balance[rush]",
            "value": 1489.9366712246856,
            "unit": "iter/sec",
            "range": "stddev: 0.0001481927937283579",
            "extra": "mean: 671.1694660002081 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBalance::test_get_balance[original]",
            "value": 472.2003191842085,
            "unit": "iter/sec",
            "range": "stddev: 0.00002902677289034926",
            "extra": "mean: 2.117745285999888 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetCode::test_get_code[rush]",
            "value": 1622.884265055872,
            "unit": "iter/sec",
            "range": "stddev: 0.000020380015043770515",
            "extra": "mean: 616.18688499982 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetCode::test_get_code[original]",
            "value": 468.39471485094026,
            "unit": "iter/sec",
            "range": "stddev: 0.000037371583585618705",
            "extra": "mean: 2.1349515020002627 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthEstimateGas::test_estimate_gas[rush]",
            "value": 1363.7722182859766,
            "unit": "iter/sec",
            "range": "stddev: 0.00003572744259593012",
            "extra": "mean: 733.2602809997297 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthEstimateGas::test_estimate_gas[original]",
            "value": 241.7153469209062,
            "unit": "iter/sec",
            "range": "stddev: 0.00005024209853419125",
            "extra": "mean: 4.137097675999939 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetLogs::test_get_logs[rush]",
            "value": 775.4598942062763,
            "unit": "iter/sec",
            "range": "stddev: 0.00006033960555172768",
            "extra": "mean: 1.2895573419996822 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetLogs::test_get_logs[original]",
            "value": 12.58473268741989,
            "unit": "iter/sec",
            "range": "stddev: 0.00017247165733544307",
            "extra": "mean: 79.46136202000002 msec\nrounds: 100"
          }
        ]
      }
    ]
  }
}