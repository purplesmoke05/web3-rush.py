window.BENCHMARK_DATA = {
  "lastUpdate": 1680872314626,
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
      },
      {
        "commit": {
          "author": {
            "email": "yosuke.otosu@gmail.com",
            "name": "purplehaze",
            "username": "purplesmoke05"
          },
          "committer": {
            "email": "yosuke.otosu@gmail.com",
            "name": "purplehaze",
            "username": "purplesmoke05"
          },
          "distinct": true,
          "id": "5a53749e51e687bb40b31991966aeddc27842d0d",
          "message": "fix",
          "timestamp": "2023-03-26T05:42:32+09:00",
          "tree_id": "79ce20bbee2c663fba65298511fb8d98c4093a34",
          "url": "https://github.com/purplesmoke05/web3-rush.py/commit/5a53749e51e687bb40b31991966aeddc27842d0d"
        },
        "date": 1679778802792,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthBlockNumber::test_block_number[rush]",
            "value": 1332.387759058384,
            "unit": "iter/sec",
            "range": "stddev: 0.0000617190078645388",
            "extra": "mean: 750.5322630003093 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthBlockNumber::test_block_number[original]",
            "value": 538.2078265055072,
            "unit": "iter/sec",
            "range": "stddev: 0.00023882410579981864",
            "extra": "mean: 1.8580183170000169 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthChainId::test_chain_id[rush]",
            "value": 1170.1690982339687,
            "unit": "iter/sec",
            "range": "stddev: 0.0004042214366620734",
            "extra": "mean: 854.5773440002904 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthChainId::test_chain_id[original]",
            "value": 586.1165276908176,
            "unit": "iter/sec",
            "range": "stddev: 0.00015231034223816872",
            "extra": "mean: 1.7061453699997173 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSyncing::test_syncing[rush]",
            "value": 1291.1818546724032,
            "unit": "iter/sec",
            "range": "stddev: 0.00012537170727452588",
            "extra": "mean: 774.4842419999145 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSyncing::test_syncing[original]",
            "value": 562.2502941723695,
            "unit": "iter/sec",
            "range": "stddev: 0.00023730564280013194",
            "extra": "mean: 1.7785673220002423 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthAccounts::test_accounts[rush]",
            "value": 1027.027578179696,
            "unit": "iter/sec",
            "range": "stddev: 0.000136744000778685",
            "extra": "mean: 973.6836880002784 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthAccounts::test_accounts[original]",
            "value": 347.78416680707903,
            "unit": "iter/sec",
            "range": "stddev: 0.0002763116308926514",
            "extra": "mean: 2.8753465380001444 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSendTransaction::test_send_transaction[rush]",
            "value": 120.56048595479231,
            "unit": "iter/sec",
            "range": "stddev: 0.003173757624758283",
            "extra": "mean: 8.294591648999983 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSendTransaction::test_send_transaction[original]",
            "value": 46.28146476240043,
            "unit": "iter/sec",
            "range": "stddev: 0.0010991190462061725",
            "extra": "mean: 21.606922017999977 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthWaitForTransactionReceipt::test_wait_for_transaction_receipt[rush]",
            "value": 885.1319953087599,
            "unit": "iter/sec",
            "range": "stddev: 0.0010713271039935494",
            "extra": "mean: 1.129775000000052 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthWaitForTransactionReceipt::test_wait_for_transaction_receipt[original]",
            "value": 337.1684619605301,
            "unit": "iter/sec",
            "range": "stddev: 0.0011528344369131738",
            "extra": "mean: 2.9658764470001415 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransaction::test_get_transaction[rush]",
            "value": 735.6968548750283,
            "unit": "iter/sec",
            "range": "stddev: 0.00028631069636590967",
            "extra": "mean: 1.3592555049999078 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransaction::test_get_transaction[original]",
            "value": 288.1787766976625,
            "unit": "iter/sec",
            "range": "stddev: 0.0003269420059764621",
            "extra": "mean: 3.4700681690002853 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransactionCount::test_get_transaction_count[rush]",
            "value": 1235.6201675723023,
            "unit": "iter/sec",
            "range": "stddev: 0.00015539532598001323",
            "extra": "mean: 809.3101960004105 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransactionCount::test_get_transaction_count[original]",
            "value": 376.7918402845172,
            "unit": "iter/sec",
            "range": "stddev: 0.00014479777690325815",
            "extra": "mean: 2.653985286000079 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBlock::test_get_block[rush]",
            "value": 1004.87873751958,
            "unit": "iter/sec",
            "range": "stddev: 0.00012997129450949982",
            "extra": "mean: 995.1449489998936 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBlock::test_get_block[original]",
            "value": 417.4669028848111,
            "unit": "iter/sec",
            "range": "stddev: 0.00030465542756093525",
            "extra": "mean: 2.3953994749996355 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBalance::test_get_balance[rush]",
            "value": 1326.1565929185638,
            "unit": "iter/sec",
            "range": "stddev: 0.00009328591999826107",
            "extra": "mean: 754.058762999648 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBalance::test_get_balance[original]",
            "value": 374.272774105015,
            "unit": "iter/sec",
            "range": "stddev: 0.00010397003055581372",
            "extra": "mean: 2.671848099000158 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetCode::test_get_code[rush]",
            "value": 1336.4376534804944,
            "unit": "iter/sec",
            "range": "stddev: 0.00007931853113382312",
            "extra": "mean: 748.2578760001957 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetCode::test_get_code[original]",
            "value": 380.2803882750756,
            "unit": "iter/sec",
            "range": "stddev: 0.00007867602930157327",
            "extra": "mean: 2.6296386319997396 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthEstimateGas::test_estimate_gas[rush]",
            "value": 1110.0465382456537,
            "unit": "iter/sec",
            "range": "stddev: 0.00011426372657374376",
            "extra": "mean: 900.8631310002784 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthEstimateGas::test_estimate_gas[original]",
            "value": 189.1905992369545,
            "unit": "iter/sec",
            "range": "stddev: 0.0003640599505844694",
            "extra": "mean: 5.285674890999928 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetLogs::test_get_logs[rush]",
            "value": 586.7913955152501,
            "unit": "iter/sec",
            "range": "stddev: 0.0003251639078538133",
            "extra": "mean: 1.7041831349996528 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetLogs::test_get_logs[original]",
            "value": 16.193669421198194,
            "unit": "iter/sec",
            "range": "stddev: 0.0012515672818805567",
            "extra": "mean: 61.752526495999575 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetVersion::test_version[rush]",
            "value": 1285.6963697186534,
            "unit": "iter/sec",
            "range": "stddev: 0.00015422607401454274",
            "extra": "mean: 777.7886160002367 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetVersion::test_version[original]",
            "value": 605.6890969253024,
            "unit": "iter/sec",
            "range": "stddev: 0.00014761140537856914",
            "extra": "mean: 1.6510120539999207 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetListening::test_listening[rush]",
            "value": 1381.0719106220274,
            "unit": "iter/sec",
            "range": "stddev: 0.000029317428133097016",
            "extra": "mean: 724.0752579998571 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetListening::test_listening[original]",
            "value": 593.0218908913736,
            "unit": "iter/sec",
            "range": "stddev: 0.00007811556910820099",
            "extra": "mean: 1.6862783909998598 msec\nrounds: 100"
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
          "id": "7749f9a9e95ea5287bd1b7c118b32efcc3240459",
          "message": "📝 fix doc (#6)",
          "timestamp": "2023-03-26T06:18:04+09:00",
          "tree_id": "af615efe27ace6a658e1bc8246f4d7b0d593906a",
          "url": "https://github.com/purplesmoke05/web3-rush.py/commit/7749f9a9e95ea5287bd1b7c118b32efcc3240459"
        },
        "date": 1679779425802,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthBlockNumber::test_block_number[rush]",
            "value": 1492.3772570857673,
            "unit": "iter/sec",
            "range": "stddev: 0.00011932842694128738",
            "extra": "mean: 670.0718569999822 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthBlockNumber::test_block_number[original]",
            "value": 735.2690638942612,
            "unit": "iter/sec",
            "range": "stddev: 0.00010478224936101854",
            "extra": "mean: 1.3600463410001566 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthChainId::test_chain_id[rush]",
            "value": 1639.7949666614322,
            "unit": "iter/sec",
            "range": "stddev: 0.00001886439046783051",
            "extra": "mean: 609.8323390002633 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthChainId::test_chain_id[original]",
            "value": 773.5947678693992,
            "unit": "iter/sec",
            "range": "stddev: 0.000023439755625289502",
            "extra": "mean: 1.292666447000613 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSyncing::test_syncing[rush]",
            "value": 1709.7248152431857,
            "unit": "iter/sec",
            "range": "stddev: 0.00001372791550603612",
            "extra": "mean: 584.8894459999769 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSyncing::test_syncing[original]",
            "value": 763.6381168663739,
            "unit": "iter/sec",
            "range": "stddev: 0.00005223370955663976",
            "extra": "mean: 1.309520802999657 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthAccounts::test_accounts[rush]",
            "value": 1273.0416140151312,
            "unit": "iter/sec",
            "range": "stddev: 0.00001708271474668763",
            "extra": "mean: 785.5202759994881 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthAccounts::test_accounts[original]",
            "value": 435.5515594454478,
            "unit": "iter/sec",
            "range": "stddev: 0.00007352960982041614",
            "extra": "mean: 2.295939432000239 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSendTransaction::test_send_transaction[rush]",
            "value": 149.1562152804764,
            "unit": "iter/sec",
            "range": "stddev: 0.002574982691695671",
            "extra": "mean: 6.704380357999696 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSendTransaction::test_send_transaction[original]",
            "value": 55.84901886686457,
            "unit": "iter/sec",
            "range": "stddev: 0.00098226374799094",
            "extra": "mean: 17.905417504000297 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthWaitForTransactionReceipt::test_wait_for_transaction_receipt[rush]",
            "value": 1214.5476309624935,
            "unit": "iter/sec",
            "range": "stddev: 0.0010960476652867334",
            "extra": "mean: 823.3518179995372 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthWaitForTransactionReceipt::test_wait_for_transaction_receipt[original]",
            "value": 423.23289883888015,
            "unit": "iter/sec",
            "range": "stddev: 0.0010596942768722327",
            "extra": "mean: 2.362765283000101 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransaction::test_get_transaction[rush]",
            "value": 968.3295315874573,
            "unit": "iter/sec",
            "range": "stddev: 0.000028979461533502688",
            "extra": "mean: 1.0327062920002277 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransaction::test_get_transaction[original]",
            "value": 372.2384299053884,
            "unit": "iter/sec",
            "range": "stddev: 0.00006131881517079588",
            "extra": "mean: 2.686450187999583 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransactionCount::test_get_transaction_count[rush]",
            "value": 1579.382889903194,
            "unit": "iter/sec",
            "range": "stddev: 0.00003440784515641373",
            "extra": "mean: 633.158688999913 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransactionCount::test_get_transaction_count[original]",
            "value": 446.0113246485066,
            "unit": "iter/sec",
            "range": "stddev: 0.00006481127522345596",
            "extra": "mean: 2.2420955360003063 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBlock::test_get_block[rush]",
            "value": 1246.6614530953022,
            "unit": "iter/sec",
            "range": "stddev: 0.000020062867746239225",
            "extra": "mean: 802.1423919999506 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBlock::test_get_block[original]",
            "value": 509.09790176335315,
            "unit": "iter/sec",
            "range": "stddev: 0.000021633805240759977",
            "extra": "mean: 1.9642587340005093 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBalance::test_get_balance[rush]",
            "value": 1599.86317970161,
            "unit": "iter/sec",
            "range": "stddev: 0.000016605438333738793",
            "extra": "mean: 625.0534499997116 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBalance::test_get_balance[original]",
            "value": 450.3270657028158,
            "unit": "iter/sec",
            "range": "stddev: 0.000020417275560171514",
            "extra": "mean: 2.220608256000162 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetCode::test_get_code[rush]",
            "value": 1580.2136168794348,
            "unit": "iter/sec",
            "range": "stddev: 0.000032093932722069426",
            "extra": "mean: 632.825834000073 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetCode::test_get_code[original]",
            "value": 445.9493347631326,
            "unit": "iter/sec",
            "range": "stddev: 0.000022035711663695573",
            "extra": "mean: 2.242407202000095 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthEstimateGas::test_estimate_gas[rush]",
            "value": 1445.6893215752907,
            "unit": "iter/sec",
            "range": "stddev: 0.00003231759525484505",
            "extra": "mean: 691.7115490002743 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthEstimateGas::test_estimate_gas[original]",
            "value": 237.96788148122948,
            "unit": "iter/sec",
            "range": "stddev: 0.00013536751204019378",
            "extra": "mean: 4.202247772999897 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetLogs::test_get_logs[rush]",
            "value": 779.0619746207404,
            "unit": "iter/sec",
            "range": "stddev: 0.0000564698196323482",
            "extra": "mean: 1.2835949290001167 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetLogs::test_get_logs[original]",
            "value": 19.73205133819024,
            "unit": "iter/sec",
            "range": "stddev: 0.0004162012533857512",
            "extra": "mean: 50.67896808400036 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetVersion::test_version[rush]",
            "value": 1591.8254769796404,
            "unit": "iter/sec",
            "range": "stddev: 0.00008871051176777065",
            "extra": "mean: 628.209570999843 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetVersion::test_version[original]",
            "value": 742.4240740399842,
            "unit": "iter/sec",
            "range": "stddev: 0.00024373663375870856",
            "extra": "mean: 1.3469390810004143 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetListening::test_listening[rush]",
            "value": 1712.697504443722,
            "unit": "iter/sec",
            "range": "stddev: 0.000021703347546610772",
            "extra": "mean: 583.8742670001125 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetListening::test_listening[original]",
            "value": 774.47808883874,
            "unit": "iter/sec",
            "range": "stddev: 0.00003454982276070983",
            "extra": "mean: 1.291192113000136 msec\nrounds: 100"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "yosuke.otosu@gmail.com",
            "name": "purplehaze",
            "username": "purplesmoke05"
          },
          "committer": {
            "email": "yosuke.otosu@gmail.com",
            "name": "purplehaze",
            "username": "purplesmoke05"
          },
          "distinct": true,
          "id": "5e9873306d25c138201d269252aed658a3e99341",
          "message": "up",
          "timestamp": "2023-03-26T07:10:33+09:00",
          "tree_id": "1ac6999ff1263807dedbad8122a593935869e8bd",
          "url": "https://github.com/purplesmoke05/web3-rush.py/commit/5e9873306d25c138201d269252aed658a3e99341"
        },
        "date": 1679782599637,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthBlockNumber::test_block_number[rush]",
            "value": 1755.7040581934737,
            "unit": "iter/sec",
            "range": "stddev: 0.000035935379971507865",
            "extra": "mean: 569.5720730001311 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthBlockNumber::test_block_number[original]",
            "value": 736.7781709108259,
            "unit": "iter/sec",
            "range": "stddev: 0.00004902565198044418",
            "extra": "mean: 1.3572606239999914 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthChainId::test_chain_id[rush]",
            "value": 1567.3927781953344,
            "unit": "iter/sec",
            "range": "stddev: 0.000031279011399184754",
            "extra": "mean: 638.0021739996664 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthChainId::test_chain_id[original]",
            "value": 762.6799131601533,
            "unit": "iter/sec",
            "range": "stddev: 0.00003598425963499396",
            "extra": "mean: 1.3111660379994987 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSyncing::test_syncing[rush]",
            "value": 1645.5994095599215,
            "unit": "iter/sec",
            "range": "stddev: 0.000015093086541519172",
            "extra": "mean: 607.6813070001208 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSyncing::test_syncing[original]",
            "value": 769.5327415740047,
            "unit": "iter/sec",
            "range": "stddev: 0.000035165706109047385",
            "extra": "mean: 1.2994898669998065 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthAccounts::test_accounts[rush]",
            "value": 1288.0784770260966,
            "unit": "iter/sec",
            "range": "stddev: 0.0000304099627737658",
            "extra": "mean: 776.3502130000575 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthAccounts::test_accounts[original]",
            "value": 419.53842800845456,
            "unit": "iter/sec",
            "range": "stddev: 0.00007315160561840302",
            "extra": "mean: 2.3835718810002504 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSendTransaction::test_send_transaction[rush]",
            "value": 148.6240342912698,
            "unit": "iter/sec",
            "range": "stddev: 0.0026329837511450843",
            "extra": "mean: 6.72838686399956 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSendTransaction::test_send_transaction[original]",
            "value": 54.368096389580984,
            "unit": "iter/sec",
            "range": "stddev: 0.0009717589585691936",
            "extra": "mean: 18.39313984499995 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthWaitForTransactionReceipt::test_wait_for_transaction_receipt[rush]",
            "value": 1226.4921740500808,
            "unit": "iter/sec",
            "range": "stddev: 0.0010254115970296709",
            "extra": "mean: 815.3333720001115 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthWaitForTransactionReceipt::test_wait_for_transaction_receipt[original]",
            "value": 400.93079580360666,
            "unit": "iter/sec",
            "range": "stddev: 0.001047700287204842",
            "extra": "mean: 2.4941960320001044 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransaction::test_get_transaction[rush]",
            "value": 946.0951090908695,
            "unit": "iter/sec",
            "range": "stddev: 0.000054562000367572594",
            "extra": "mean: 1.056976185999872 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransaction::test_get_transaction[original]",
            "value": 359.2286837267473,
            "unit": "iter/sec",
            "range": "stddev: 0.000026666341729860357",
            "extra": "mean: 2.783742070999722 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransactionCount::test_get_transaction_count[rush]",
            "value": 1568.3137119745677,
            "unit": "iter/sec",
            "range": "stddev: 0.00006984989906927552",
            "extra": "mean: 637.6275310001347 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransactionCount::test_get_transaction_count[original]",
            "value": 446.5897713958945,
            "unit": "iter/sec",
            "range": "stddev: 0.00008674658170369177",
            "extra": "mean: 2.2391914549997978 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBlock::test_get_block[rush]",
            "value": 1263.7836794422335,
            "unit": "iter/sec",
            "range": "stddev: 0.000017626182627818874",
            "extra": "mean: 791.2746589996686 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBlock::test_get_block[original]",
            "value": 509.2856037547755,
            "unit": "iter/sec",
            "range": "stddev: 0.00006160176276093836",
            "extra": "mean: 1.9635347879998337 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBalance::test_get_balance[rush]",
            "value": 1577.8286820726526,
            "unit": "iter/sec",
            "range": "stddev: 0.00002314079466142746",
            "extra": "mean: 633.7823689999027 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBalance::test_get_balance[original]",
            "value": 449.3298608408869,
            "unit": "iter/sec",
            "range": "stddev: 0.00004240834983642433",
            "extra": "mean: 2.2255364869999426 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetCode::test_get_code[rush]",
            "value": 1569.2250832759933,
            "unit": "iter/sec",
            "range": "stddev: 0.00001857111721986271",
            "extra": "mean: 637.2572110001897 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetCode::test_get_code[original]",
            "value": 448.1446694168176,
            "unit": "iter/sec",
            "range": "stddev: 0.000040559145716422804",
            "extra": "mean: 2.2314222800001744 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthEstimateGas::test_estimate_gas[rush]",
            "value": 1391.1907724632897,
            "unit": "iter/sec",
            "range": "stddev: 0.0000137461141125263",
            "extra": "mean: 718.8086779999026 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthEstimateGas::test_estimate_gas[original]",
            "value": 230.47690972438926,
            "unit": "iter/sec",
            "range": "stddev: 0.00010378355355368108",
            "extra": "mean: 4.338829434999923 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetLogs::test_get_logs[rush]",
            "value": 763.8916076005675,
            "unit": "iter/sec",
            "range": "stddev: 0.00006545884804597623",
            "extra": "mean: 1.3090862499996092 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetLogs::test_get_logs[original]",
            "value": 19.19657510690779,
            "unit": "iter/sec",
            "range": "stddev: 0.0005437101553959309",
            "extra": "mean: 52.09262560799999 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetVersion::test_version[rush]",
            "value": 1556.6799991377077,
            "unit": "iter/sec",
            "range": "stddev: 0.0001374284864879598",
            "extra": "mean: 642.3927850000837 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetVersion::test_version[original]",
            "value": 751.655111482719,
            "unit": "iter/sec",
            "range": "stddev: 0.00011506819936227554",
            "extra": "mean: 1.3303973919998953 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetListening::test_listening[rush]",
            "value": 1660.2791880002137,
            "unit": "iter/sec",
            "range": "stddev: 0.000016338426078167013",
            "extra": "mean: 602.3083389995918 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetListening::test_listening[original]",
            "value": 779.5240776638141,
            "unit": "iter/sec",
            "range": "stddev: 0.000019155906666428674",
            "extra": "mean: 1.2828340119999098 msec\nrounds: 100"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "yosuke.otosu@gmail.com",
            "name": "purplehaze",
            "username": "purplesmoke05"
          },
          "committer": {
            "email": "yosuke.otosu@gmail.com",
            "name": "purplehaze",
            "username": "purplesmoke05"
          },
          "distinct": true,
          "id": "452e824f37ce856a4b6278755f2358806125b0f6",
          "message": "📝 Update README",
          "timestamp": "2023-03-26T08:33:01+09:00",
          "tree_id": "ae0564f93c924c7da5450836d61d111fc98ea5b6",
          "url": "https://github.com/purplesmoke05/web3-rush.py/commit/452e824f37ce856a4b6278755f2358806125b0f6"
        },
        "date": 1679787525149,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthBlockNumber::test_block_number[rush]",
            "value": 1584.9740855619796,
            "unit": "iter/sec",
            "range": "stddev: 0.00005258065856208922",
            "extra": "mean: 630.9251420003079 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthBlockNumber::test_block_number[original]",
            "value": 731.6458907610777,
            "unit": "iter/sec",
            "range": "stddev: 0.0000478578070953957",
            "extra": "mean: 1.3667814069997348 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthChainId::test_chain_id[rush]",
            "value": 1615.3844824256375,
            "unit": "iter/sec",
            "range": "stddev: 0.000026767271110889594",
            "extra": "mean: 619.0476700001568 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthChainId::test_chain_id[original]",
            "value": 757.1304484344007,
            "unit": "iter/sec",
            "range": "stddev: 0.00005052529010503963",
            "extra": "mean: 1.3207763630003342 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSyncing::test_syncing[rush]",
            "value": 1643.9163412876483,
            "unit": "iter/sec",
            "range": "stddev: 0.00005369876053780704",
            "extra": "mean: 608.3034610001617 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSyncing::test_syncing[original]",
            "value": 764.3160921162629,
            "unit": "iter/sec",
            "range": "stddev: 0.000021804591134803854",
            "extra": "mean: 1.3083592119998002 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthAccounts::test_accounts[rush]",
            "value": 1234.778911230965,
            "unit": "iter/sec",
            "range": "stddev: 0.00007755661764820884",
            "extra": "mean: 809.861579999847 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthAccounts::test_accounts[original]",
            "value": 423.9878381497416,
            "unit": "iter/sec",
            "range": "stddev: 0.00006734522724732332",
            "extra": "mean: 2.3585582179997004 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSendTransaction::test_send_transaction[rush]",
            "value": 148.16966523717073,
            "unit": "iter/sec",
            "range": "stddev: 0.002569785582715634",
            "extra": "mean: 6.749019770000359 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSendTransaction::test_send_transaction[original]",
            "value": 55.256455306920714,
            "unit": "iter/sec",
            "range": "stddev: 0.0009552807091940626",
            "extra": "mean: 18.097433040999874 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthWaitForTransactionReceipt::test_wait_for_transaction_receipt[rush]",
            "value": 1246.5142289121768,
            "unit": "iter/sec",
            "range": "stddev: 0.0010295856256233257",
            "extra": "mean: 802.2371320002435 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthWaitForTransactionReceipt::test_wait_for_transaction_receipt[original]",
            "value": 403.15337685879393,
            "unit": "iter/sec",
            "range": "stddev: 0.0010778549922222708",
            "extra": "mean: 2.4804455509999457 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransaction::test_get_transaction[rush]",
            "value": 957.3856952209067,
            "unit": "iter/sec",
            "range": "stddev: 0.000019896020712580404",
            "extra": "mean: 1.0445111149997501 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransaction::test_get_transaction[original]",
            "value": 359.7648930667687,
            "unit": "iter/sec",
            "range": "stddev: 0.00013299558201653662",
            "extra": "mean: 2.779593059999911 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransactionCount::test_get_transaction_count[rush]",
            "value": 1634.6180669611679,
            "unit": "iter/sec",
            "range": "stddev: 0.0000829491522958682",
            "extra": "mean: 611.7637019998483 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransactionCount::test_get_transaction_count[original]",
            "value": 445.6599754913276,
            "unit": "iter/sec",
            "range": "stddev: 0.000038468697443433286",
            "extra": "mean: 2.2438631579996127 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBlock::test_get_block[rush]",
            "value": 1226.419777180517,
            "unit": "iter/sec",
            "range": "stddev: 0.000029096540176821348",
            "extra": "mean: 815.3815020000366 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBlock::test_get_block[original]",
            "value": 501.5078975871035,
            "unit": "iter/sec",
            "range": "stddev: 0.00002285610208102164",
            "extra": "mean: 1.9939865450001548 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBalance::test_get_balance[rush]",
            "value": 1597.2474978868502,
            "unit": "iter/sec",
            "range": "stddev: 0.000019312384662499946",
            "extra": "mean: 626.0770490002301 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBalance::test_get_balance[original]",
            "value": 446.67436915533347,
            "unit": "iter/sec",
            "range": "stddev: 0.000026328698189352323",
            "extra": "mean: 2.238767363999443 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetCode::test_get_code[rush]",
            "value": 1551.5765664138364,
            "unit": "iter/sec",
            "range": "stddev: 0.000019299432751201067",
            "extra": "mean: 644.5057379999639 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetCode::test_get_code[original]",
            "value": 446.44413738106624,
            "unit": "iter/sec",
            "range": "stddev: 0.00003620884891269183",
            "extra": "mean: 2.2399218989999667 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthEstimateGas::test_estimate_gas[rush]",
            "value": 1375.5263363836586,
            "unit": "iter/sec",
            "range": "stddev: 0.000014717730336708436",
            "extra": "mean: 726.994440999988 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthEstimateGas::test_estimate_gas[original]",
            "value": 231.0593189901517,
            "unit": "iter/sec",
            "range": "stddev: 0.00005422329131998548",
            "extra": "mean: 4.327892960000554 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetLogs::test_get_logs[rush]",
            "value": 785.61326106784,
            "unit": "iter/sec",
            "range": "stddev: 0.000025122300780927137",
            "extra": "mean: 1.2728909369996586 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetLogs::test_get_logs[original]",
            "value": 19.69960335004477,
            "unit": "iter/sec",
            "range": "stddev: 0.0005145246741192047",
            "extra": "mean: 50.76244339700003 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetVersion::test_version[rush]",
            "value": 1406.481954367098,
            "unit": "iter/sec",
            "range": "stddev: 0.00019743566747472828",
            "extra": "mean: 710.9938359998296 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetVersion::test_version[original]",
            "value": 746.2566141396445,
            "unit": "iter/sec",
            "range": "stddev: 0.00021131478693646948",
            "extra": "mean: 1.340021623999803 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetListening::test_listening[rush]",
            "value": 1673.1486117030754,
            "unit": "iter/sec",
            "range": "stddev: 0.000018147003840780915",
            "extra": "mean: 597.6755400000684 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetListening::test_listening[original]",
            "value": 769.7573424650342,
            "unit": "iter/sec",
            "range": "stddev: 0.00002511195648722118",
            "extra": "mean: 1.299110700000142 msec\nrounds: 100"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "yosuke.otosu@gmail.com",
            "name": "purplehaze",
            "username": "purplesmoke05"
          },
          "committer": {
            "email": "yosuke.otosu@gmail.com",
            "name": "purplehaze",
            "username": "purplesmoke05"
          },
          "distinct": true,
          "id": "34167f3e1f699c0046df3756863d58184f2a8d6e",
          "message": "update root method",
          "timestamp": "2023-03-28T02:55:48+09:00",
          "tree_id": "5eaf4f768faad68f7e8be841632b8820790c2e8a",
          "url": "https://github.com/purplesmoke05/web3-rush.py/commit/34167f3e1f699c0046df3756863d58184f2a8d6e"
        },
        "date": 1679940132382,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthBlockNumber::test_block_number[rush]",
            "value": 1325.260219150314,
            "unit": "iter/sec",
            "range": "stddev: 0.00014613201177192002",
            "extra": "mean: 754.5687900004623 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthBlockNumber::test_block_number[original]",
            "value": 598.8872022994635,
            "unit": "iter/sec",
            "range": "stddev: 0.00012082428010778521",
            "extra": "mean: 1.669763514999886 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthChainId::test_chain_id[rush]",
            "value": 1377.2061163517926,
            "unit": "iter/sec",
            "range": "stddev: 0.00009139986536559072",
            "extra": "mean: 726.1077249997925 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthChainId::test_chain_id[original]",
            "value": 620.2455515296168,
            "unit": "iter/sec",
            "range": "stddev: 0.00010189032381020981",
            "extra": "mean: 1.612264686999936 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSyncing::test_syncing[rush]",
            "value": 1460.998716690572,
            "unit": "iter/sec",
            "range": "stddev: 0.00010196039071195618",
            "extra": "mean: 684.4632980001393 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSyncing::test_syncing[original]",
            "value": 599.4516720842953,
            "unit": "iter/sec",
            "range": "stddev: 0.00023751413094093185",
            "extra": "mean: 1.6681911929997568 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthAccounts::test_accounts[rush]",
            "value": 1079.9993723895882,
            "unit": "iter/sec",
            "range": "stddev: 0.0001965984937161567",
            "extra": "mean: 925.9264640010088 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthAccounts::test_accounts[original]",
            "value": 350.14290179171877,
            "unit": "iter/sec",
            "range": "stddev: 0.0003316429278608076",
            "extra": "mean: 2.85597678799968 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSendTransaction::test_send_transaction[rush]",
            "value": 143.78876116870148,
            "unit": "iter/sec",
            "range": "stddev: 0.002468954215650192",
            "extra": "mean: 6.954646467999964 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSendTransaction::test_send_transaction[original]",
            "value": 48.89218171735326,
            "unit": "iter/sec",
            "range": "stddev: 0.0011472059951965327",
            "extra": "mean: 20.453167865999944 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthWaitForTransactionReceipt::test_wait_for_transaction_receipt[rush]",
            "value": 1118.3047142891708,
            "unit": "iter/sec",
            "range": "stddev: 0.0010385879666149121",
            "extra": "mean: 894.2106629995124 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthWaitForTransactionReceipt::test_wait_for_transaction_receipt[original]",
            "value": 338.4126770440179,
            "unit": "iter/sec",
            "range": "stddev: 0.0011851693513817109",
            "extra": "mean: 2.9549720439991916 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransaction::test_get_transaction[rush]",
            "value": 832.3664287441438,
            "unit": "iter/sec",
            "range": "stddev: 0.0001253959058237334",
            "extra": "mean: 1.2013939600000185 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransaction::test_get_transaction[original]",
            "value": 314.22488566271045,
            "unit": "iter/sec",
            "range": "stddev: 0.00018351452336715413",
            "extra": "mean: 3.1824341280002955 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransactionCount::test_get_transaction_count[rush]",
            "value": 1421.0297476156188,
            "unit": "iter/sec",
            "range": "stddev: 0.00008716270742701018",
            "extra": "mean: 703.7150359997213 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransactionCount::test_get_transaction_count[original]",
            "value": 388.22761546419196,
            "unit": "iter/sec",
            "range": "stddev: 0.00015727421474441517",
            "extra": "mean: 2.5758085209995443 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBlock::test_get_block[rush]",
            "value": 1062.374938218987,
            "unit": "iter/sec",
            "range": "stddev: 0.00007046645894285624",
            "extra": "mean: 941.287264999346 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBlock::test_get_block[original]",
            "value": 446.5756337689298,
            "unit": "iter/sec",
            "range": "stddev: 0.00011024740352175636",
            "extra": "mean: 2.239262342999723 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBalance::test_get_balance[rush]",
            "value": 1400.2831661019864,
            "unit": "iter/sec",
            "range": "stddev: 0.00004893919286913232",
            "extra": "mean: 714.1412710000168 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBalance::test_get_balance[original]",
            "value": 393.15309573020363,
            "unit": "iter/sec",
            "range": "stddev: 0.0001164148974568116",
            "extra": "mean: 2.5435384099995417 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetCode::test_get_code[rush]",
            "value": 1348.6285341277276,
            "unit": "iter/sec",
            "range": "stddev: 0.00016359359903727824",
            "extra": "mean: 741.4940249998381 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetCode::test_get_code[original]",
            "value": 386.60985100875826,
            "unit": "iter/sec",
            "range": "stddev: 0.00021002829568595897",
            "extra": "mean: 2.5865869619999557 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthEstimateGas::test_estimate_gas[rush]",
            "value": 1256.8284890029943,
            "unit": "iter/sec",
            "range": "stddev: 0.00005095990687408488",
            "extra": "mean: 795.653510999955 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthEstimateGas::test_estimate_gas[original]",
            "value": 201.5183305920127,
            "unit": "iter/sec",
            "range": "stddev: 0.00026714830712324745",
            "extra": "mean: 4.962327729999743 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetLogs::test_get_logs[rush]",
            "value": 685.2614541382244,
            "unit": "iter/sec",
            "range": "stddev: 0.00018043637563130668",
            "extra": "mean: 1.4592970229991806 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetLogs::test_get_logs[original]",
            "value": 25.15002116898414,
            "unit": "iter/sec",
            "range": "stddev: 0.00176917904768527",
            "extra": "mean: 39.761397943999896 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetVersion::test_version[rush]",
            "value": 1185.4779444956728,
            "unit": "iter/sec",
            "range": "stddev: 0.00022620155491927236",
            "extra": "mean: 843.5416320000968 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetVersion::test_version[original]",
            "value": 600.2911031268559,
            "unit": "iter/sec",
            "range": "stddev: 0.00017802534452749776",
            "extra": "mean: 1.6658584389991802 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetListening::test_listening[rush]",
            "value": 1461.9745585409785,
            "unit": "iter/sec",
            "range": "stddev: 0.00007585075997947959",
            "extra": "mean: 684.006430999716 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetListening::test_listening[original]",
            "value": 619.9650384107093,
            "unit": "iter/sec",
            "range": "stddev: 0.00017255664341000874",
            "extra": "mean: 1.6129941820001932 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3Api::test_api[rush]",
            "value": 545666.2638818565,
            "unit": "iter/sec",
            "range": "stddev: 6.187559269496352e-7",
            "extra": "mean: 1.832622000279116 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3Api::test_api[original]",
            "value": 1041762.1610544515,
            "unit": "iter/sec",
            "range": "stddev: 5.636312186215996e-8",
            "extra": "mean: 959.9120004395445 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToHex::test_to_hex[rush]",
            "value": 95304.99026935594,
            "unit": "iter/sec",
            "range": "stddev: 0.000002491526708242988",
            "extra": "mean: 10.492629999475866 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToHex::test_to_hex[original]",
            "value": 107737.57239383667,
            "unit": "iter/sec",
            "range": "stddev: 0.000001489146269696254",
            "extra": "mean: 9.281812999688555 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToInt::test_to_int[rush]",
            "value": 118660.48920611426,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014886988605028304",
            "extra": "mean: 8.427404999679311 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToInt::test_to_int[original]",
            "value": 147682.54338812907,
            "unit": "iter/sec",
            "range": "stddev: 8.97586476693118e-7",
            "extra": "mean: 6.771280999487317 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToWei::test_to_wei[rush]",
            "value": 58634.48837059804,
            "unit": "iter/sec",
            "range": "stddev: 0.00000546034559038498",
            "extra": "mean: 17.05480900045586 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToWei::test_to_wei[original]",
            "value": 58355.95745142179,
            "unit": "iter/sec",
            "range": "stddev: 0.00001554250123712459",
            "extra": "mean: 17.13621100009277 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3FromWei::test_from_wei[rush]",
            "value": 130225.37454073457,
            "unit": "iter/sec",
            "range": "stddev: 0.0000014885684677039448",
            "extra": "mean: 7.678995000219402 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3FromWei::test_from_wei[original]",
            "value": 141418.9325221014,
            "unit": "iter/sec",
            "range": "stddev: 7.769633035510491e-7",
            "extra": "mean: 7.0711889997028265 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3IsAddress::test_is_address[rush]",
            "value": 128210.14154639016,
            "unit": "iter/sec",
            "range": "stddev: 0.0000022521354892179457",
            "extra": "mean: 7.799695000244356 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3IsAddress::test_is_address[original]",
            "value": 16644.82311034691,
            "unit": "iter/sec",
            "range": "stddev: 0.000015442866500252517",
            "extra": "mean: 60.078740000449216 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3IsChecksumAddress::test_is_checksum_address[rush]",
            "value": 27354.73003899974,
            "unit": "iter/sec",
            "range": "stddev: 0.0000029191117740586895",
            "extra": "mean: 36.55674900005579 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3IsChecksumAddress::test_is_checksum_address[original]",
            "value": 9914.81694928037,
            "unit": "iter/sec",
            "range": "stddev: 0.000008506626885101468",
            "extra": "mean: 100.85914900048465 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToChecksumAddress::test_to_checksum_address[rush]",
            "value": 26367.295904515573,
            "unit": "iter/sec",
            "range": "stddev: 0.0000029582268389579283",
            "extra": "mean: 37.92577000012898 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToChecksumAddress::test_to_checksum_address[original]",
            "value": 10220.788972444287,
            "unit": "iter/sec",
            "range": "stddev: 0.00002176703183923341",
            "extra": "mean: 97.8398049990119 usec\nrounds: 100"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "yosuke.otosu@gmail.com",
            "name": "purplehaze",
            "username": "purplesmoke05"
          },
          "committer": {
            "email": "yosuke.otosu@gmail.com",
            "name": "purplehaze",
            "username": "purplesmoke05"
          },
          "distinct": true,
          "id": "d95b2f3f3a33b87ddc472e8c6075f5671e2ac03a",
          "message": "📝 fix README usage",
          "timestamp": "2023-03-29T00:11:58+09:00",
          "tree_id": "074a4cb5671136f599d7dfce3d446d1c6baceaff",
          "url": "https://github.com/purplesmoke05/web3-rush.py/commit/d95b2f3f3a33b87ddc472e8c6075f5671e2ac03a"
        },
        "date": 1680016726597,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthBlockNumber::test_block_number[rush]",
            "value": 1369.381085349317,
            "unit": "iter/sec",
            "range": "stddev: 0.00009441084897838644",
            "extra": "mean: 730.2569100002643 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthBlockNumber::test_block_number[original]",
            "value": 583.7949248504577,
            "unit": "iter/sec",
            "range": "stddev: 0.00012271080331770746",
            "extra": "mean: 1.712930272999813 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthChainId::test_chain_id[rush]",
            "value": 1451.8352297793908,
            "unit": "iter/sec",
            "range": "stddev: 0.00005930481693043443",
            "extra": "mean: 688.7833960000762 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthChainId::test_chain_id[original]",
            "value": 614.38403144125,
            "unit": "iter/sec",
            "range": "stddev: 0.00013850522509546142",
            "extra": "mean: 1.6276464700004567 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSyncing::test_syncing[rush]",
            "value": 1338.383292621371,
            "unit": "iter/sec",
            "range": "stddev: 0.00006281087647625479",
            "extra": "mean: 747.1701159997224 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSyncing::test_syncing[original]",
            "value": 614.0745807947789,
            "unit": "iter/sec",
            "range": "stddev: 0.00007700721036453099",
            "extra": "mean: 1.628466689999982 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthAccounts::test_accounts[rush]",
            "value": 1079.6453628414847,
            "unit": "iter/sec",
            "range": "stddev: 0.00011661533397191323",
            "extra": "mean: 926.2300700001447 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthAccounts::test_accounts[original]",
            "value": 359.85033119417903,
            "unit": "iter/sec",
            "range": "stddev: 0.00006538785606755886",
            "extra": "mean: 2.7789331100000823 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSendTransaction::test_send_transaction[rush]",
            "value": 123.83603180964494,
            "unit": "iter/sec",
            "range": "stddev: 0.0032009757476298495",
            "extra": "mean: 8.075194153000268 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSendTransaction::test_send_transaction[original]",
            "value": 47.210904469203136,
            "unit": "iter/sec",
            "range": "stddev: 0.0009435553290847387",
            "extra": "mean: 21.181547170999977 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthWaitForTransactionReceipt::test_wait_for_transaction_receipt[rush]",
            "value": 1040.1675674170053,
            "unit": "iter/sec",
            "range": "stddev: 0.0010634637177998007",
            "extra": "mean: 961.3835609999342 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthWaitForTransactionReceipt::test_wait_for_transaction_receipt[original]",
            "value": 347.24620270741974,
            "unit": "iter/sec",
            "range": "stddev: 0.0010792832747096607",
            "extra": "mean: 2.8798011099996756 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransaction::test_get_transaction[rush]",
            "value": 797.1483166863715,
            "unit": "iter/sec",
            "range": "stddev: 0.00007951879523390876",
            "extra": "mean: 1.2544716950000634 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransaction::test_get_transaction[original]",
            "value": 303.69712500984355,
            "unit": "iter/sec",
            "range": "stddev: 0.00016664231182020138",
            "extra": "mean: 3.2927542529998846 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransactionCount::test_get_transaction_count[rush]",
            "value": 1304.353977533311,
            "unit": "iter/sec",
            "range": "stddev: 0.00010292664703233451",
            "extra": "mean: 766.6630510002506 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransactionCount::test_get_transaction_count[original]",
            "value": 384.5609080129579,
            "unit": "iter/sec",
            "range": "stddev: 0.00015759310160475324",
            "extra": "mean: 2.600368314000093 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBlock::test_get_block[rush]",
            "value": 1036.2804575792916,
            "unit": "iter/sec",
            "range": "stddev: 0.00009811834777436947",
            "extra": "mean: 964.9897310000028 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBlock::test_get_block[original]",
            "value": 441.9129322671877,
            "unit": "iter/sec",
            "range": "stddev: 0.00006806594584989186",
            "extra": "mean: 2.262889195999776 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBalance::test_get_balance[rush]",
            "value": 1470.4666363530373,
            "unit": "iter/sec",
            "range": "stddev: 0.00004450761552469206",
            "extra": "mean: 680.0562320000267 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBalance::test_get_balance[original]",
            "value": 380.97684518090847,
            "unit": "iter/sec",
            "range": "stddev: 0.0001825519958707829",
            "extra": "mean: 2.624831437000182 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetCode::test_get_code[rush]",
            "value": 1333.9520220598959,
            "unit": "iter/sec",
            "range": "stddev: 0.00007932969795371116",
            "extra": "mean: 749.6521489999282 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetCode::test_get_code[original]",
            "value": 386.4011537465431,
            "unit": "iter/sec",
            "range": "stddev: 0.00004734057130660239",
            "extra": "mean: 2.587983991000044 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthEstimateGas::test_estimate_gas[rush]",
            "value": 1196.9461876229955,
            "unit": "iter/sec",
            "range": "stddev: 0.000036814982231722915",
            "extra": "mean: 835.4594469997778 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthEstimateGas::test_estimate_gas[original]",
            "value": 199.91170683561964,
            "unit": "iter/sec",
            "range": "stddev: 0.00007549050824367021",
            "extra": "mean: 5.0022083040002485 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetLogs::test_get_logs[rush]",
            "value": 659.5314961560799,
            "unit": "iter/sec",
            "range": "stddev: 0.00009133218379980015",
            "extra": "mean: 1.5162278160000824 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetLogs::test_get_logs[original]",
            "value": 16.386482192784996,
            "unit": "iter/sec",
            "range": "stddev: 0.0004339731231189873",
            "extra": "mean: 61.025910762000024 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetVersion::test_version[rush]",
            "value": 1244.7924930726795,
            "unit": "iter/sec",
            "range": "stddev: 0.00012965497643729624",
            "extra": "mean: 803.3467470000346 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetVersion::test_version[original]",
            "value": 604.5618656851,
            "unit": "iter/sec",
            "range": "stddev: 0.0001190397182264291",
            "extra": "mean: 1.654090435999933 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetListening::test_listening[rush]",
            "value": 1400.2872543675587,
            "unit": "iter/sec",
            "range": "stddev: 0.0000465983579236911",
            "extra": "mean: 714.1391859998407 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetListening::test_listening[original]",
            "value": 602.8349916612651,
            "unit": "iter/sec",
            "range": "stddev: 0.00012902531921401027",
            "extra": "mean: 1.6588287239999886 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3Api::test_api[rush]",
            "value": 534864.6150132392,
            "unit": "iter/sec",
            "range": "stddev: 5.160078767297009e-7",
            "extra": "mean: 1.8696320001936328 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3Api::test_api[original]",
            "value": 897328.562925615,
            "unit": "iter/sec",
            "range": "stddev: 5.99367875382312e-8",
            "extra": "mean: 1.1144190002596588 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToHex::test_to_hex[rush]",
            "value": 86721.24323504606,
            "unit": "iter/sec",
            "range": "stddev: 3.402004758008221e-7",
            "extra": "mean: 11.531200000092667 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToHex::test_to_hex[original]",
            "value": 101719.31065411477,
            "unit": "iter/sec",
            "range": "stddev: 2.9744066840818966e-7",
            "extra": "mean: 9.830974999431419 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToInt::test_to_int[rush]",
            "value": 106663.65163929398,
            "unit": "iter/sec",
            "range": "stddev: 4.1523724388621967e-7",
            "extra": "mean: 9.37526500013064 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToInt::test_to_int[original]",
            "value": 132061.4407896554,
            "unit": "iter/sec",
            "range": "stddev: 2.306672653686947e-7",
            "extra": "mean: 7.572233000189499 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToWei::test_to_wei[rush]",
            "value": 56484.06254011955,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012804353238016178",
            "extra": "mean: 17.704109000476365 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToWei::test_to_wei[original]",
            "value": 57834.88960250373,
            "unit": "iter/sec",
            "range": "stddev: 4.835086515135281e-7",
            "extra": "mean: 17.290601000070183 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3FromWei::test_from_wei[rush]",
            "value": 120970.98571803451,
            "unit": "iter/sec",
            "range": "stddev: 3.38933957739588e-7",
            "extra": "mean: 8.266444999719624 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3FromWei::test_from_wei[original]",
            "value": 127327.92045266299,
            "unit": "iter/sec",
            "range": "stddev: 3.123157495782792e-7",
            "extra": "mean: 7.853737000061756 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3IsAddress::test_is_address[rush]",
            "value": 122136.79296239068,
            "unit": "iter/sec",
            "range": "stddev: 5.792179471978709e-7",
            "extra": "mean: 8.187541000097553 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3IsAddress::test_is_address[original]",
            "value": 17166.43244215147,
            "unit": "iter/sec",
            "range": "stddev: 0.0000017513375453322377",
            "extra": "mean: 58.25322199996208 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3IsChecksumAddress::test_is_checksum_address[rush]",
            "value": 25124.74624943661,
            "unit": "iter/sec",
            "range": "stddev: 0.0000011670181350855535",
            "extra": "mean: 39.80139700007612 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3IsChecksumAddress::test_is_checksum_address[original]",
            "value": 9771.03647482235,
            "unit": "iter/sec",
            "range": "stddev: 0.0000016977297045415982",
            "extra": "mean: 102.34328799987225 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToChecksumAddress::test_to_checksum_address[rush]",
            "value": 24508.953316534236,
            "unit": "iter/sec",
            "range": "stddev: 9.347065491890198e-7",
            "extra": "mean: 40.80141600030629 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToChecksumAddress::test_to_checksum_address[original]",
            "value": 10213.303516172458,
            "unit": "iter/sec",
            "range": "stddev: 0.000001371526336924263",
            "extra": "mean: 97.91151300032652 usec\nrounds: 100"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "yosuke.otosu@gmail.com",
            "name": "purplehaze",
            "username": "purplesmoke05"
          },
          "committer": {
            "email": "yosuke.otosu@gmail.com",
            "name": "purplehaze",
            "username": "purplesmoke05"
          },
          "distinct": true,
          "id": "110d808011858690f5db88397b202c3b563d9c0c",
          "message": "🔧 Add eth account module",
          "timestamp": "2023-04-07T21:51:43+09:00",
          "tree_id": "888432c088e22d505a26965b96e2226ec635862e",
          "url": "https://github.com/purplesmoke05/web3-rush.py/commit/110d808011858690f5db88397b202c3b563d9c0c"
        },
        "date": 1680872314135,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthBlockNumber::test_block_number[rush]",
            "value": 1596.1872169647604,
            "unit": "iter/sec",
            "range": "stddev: 0.00003605213848712011",
            "extra": "mean: 626.492925999969 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthBlockNumber::test_block_number[original]",
            "value": 718.2754728798311,
            "unit": "iter/sec",
            "range": "stddev: 0.00006117419133743207",
            "extra": "mean: 1.3922235100004627 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthChainId::test_chain_id[rush]",
            "value": 1630.4534690220346,
            "unit": "iter/sec",
            "range": "stddev: 0.00002843955410069089",
            "extra": "mean: 613.3263040004522 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthChainId::test_chain_id[original]",
            "value": 762.3403280283646,
            "unit": "iter/sec",
            "range": "stddev: 0.000025210790706366764",
            "extra": "mean: 1.3117500979992656 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSyncing::test_syncing[rush]",
            "value": 1568.3384609406382,
            "unit": "iter/sec",
            "range": "stddev: 0.00004841437636016696",
            "extra": "mean: 637.6174689997924 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSyncing::test_syncing[original]",
            "value": 760.5297415153697,
            "unit": "iter/sec",
            "range": "stddev: 0.000022008676860442965",
            "extra": "mean: 1.314872970000465 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthAccounts::test_accounts[rush]",
            "value": 1263.2145697219655,
            "unit": "iter/sec",
            "range": "stddev: 0.000027077433126028785",
            "extra": "mean: 791.6311480005334 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthAccounts::test_accounts[original]",
            "value": 420.9880091217812,
            "unit": "iter/sec",
            "range": "stddev: 0.00006397497838072407",
            "extra": "mean: 2.375364567000588 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSendTransaction::test_send_transaction[rush]",
            "value": 399.86348852433247,
            "unit": "iter/sec",
            "range": "stddev: 0.00007664008302056693",
            "extra": "mean: 2.500853488000189 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSendTransaction::test_send_transaction[original]",
            "value": 67.94930118169879,
            "unit": "iter/sec",
            "range": "stddev: 0.00037752099313780296",
            "extra": "mean: 14.716854811000417 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthWaitForTransactionReceipt::test_wait_for_transaction_receipt[rush]",
            "value": 1350.5546992847253,
            "unit": "iter/sec",
            "range": "stddev: 0.000015277043467319424",
            "extra": "mean: 740.4365040006269 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthWaitForTransactionReceipt::test_wait_for_transaction_receipt[original]",
            "value": 419.98723007835366,
            "unit": "iter/sec",
            "range": "stddev: 0.000056874405926011464",
            "extra": "mean: 2.3810247749995597 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransaction::test_get_transaction[rush]",
            "value": 619.4619037236699,
            "unit": "iter/sec",
            "range": "stddev: 0.000029461750458711624",
            "extra": "mean: 1.6143042759996433 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransaction::test_get_transaction[original]",
            "value": 320.52860480011054,
            "unit": "iter/sec",
            "range": "stddev: 0.00002573267747630113",
            "extra": "mean: 3.119846357000256 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransactionCount::test_get_transaction_count[rush]",
            "value": 1591.0180311650283,
            "unit": "iter/sec",
            "range": "stddev: 0.000019901458612438124",
            "extra": "mean: 628.5283890011897 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransactionCount::test_get_transaction_count[original]",
            "value": 446.4888266902611,
            "unit": "iter/sec",
            "range": "stddev: 0.00004408708549379822",
            "extra": "mean: 2.2396977040003776 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBlock::test_get_block[rush]",
            "value": 1191.5404375142377,
            "unit": "iter/sec",
            "range": "stddev: 0.00004709572180922557",
            "extra": "mean: 839.2497379998076 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBlock::test_get_block[original]",
            "value": 505.152496870404,
            "unit": "iter/sec",
            "range": "stddev: 0.00003913149190416941",
            "extra": "mean: 1.979600231999939 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBalance::test_get_balance[rush]",
            "value": 1577.8064806369098,
            "unit": "iter/sec",
            "range": "stddev: 0.0000170456305543091",
            "extra": "mean: 633.7912870001219 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBalance::test_get_balance[original]",
            "value": 444.1303958935672,
            "unit": "iter/sec",
            "range": "stddev: 0.00005026790894932661",
            "extra": "mean: 2.2515909950005835 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetCode::test_get_code[rush]",
            "value": 1669.7437567320214,
            "unit": "iter/sec",
            "range": "stddev: 0.00004254303789037223",
            "extra": "mean: 598.8942889998725 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetCode::test_get_code[original]",
            "value": 443.30013508859736,
            "unit": "iter/sec",
            "range": "stddev: 0.00004720159723374547",
            "extra": "mean: 2.2558080200001314 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthEstimateGas::test_estimate_gas[rush]",
            "value": 1419.8261554064893,
            "unit": "iter/sec",
            "range": "stddev: 0.000024147404207712153",
            "extra": "mean: 704.311577999988 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthEstimateGas::test_estimate_gas[original]",
            "value": 228.43502701450004,
            "unit": "iter/sec",
            "range": "stddev: 0.00005828167791735121",
            "extra": "mean: 4.377612370000179 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetLogs::test_get_logs[rush]",
            "value": 779.1709600415165,
            "unit": "iter/sec",
            "range": "stddev: 0.00007720478528740651",
            "extra": "mean: 1.2834153879999803 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetLogs::test_get_logs[original]",
            "value": 19.55718387969698,
            "unit": "iter/sec",
            "range": "stddev: 0.0005024893429305909",
            "extra": "mean: 51.13210604099993 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetVersion::test_version[rush]",
            "value": 1467.4290805979977,
            "unit": "iter/sec",
            "range": "stddev: 0.00013810128889541615",
            "extra": "mean: 681.463937999979 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetVersion::test_version[original]",
            "value": 752.1936586595236,
            "unit": "iter/sec",
            "range": "stddev: 0.00007268036578056238",
            "extra": "mean: 1.3294448690010086 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetListening::test_listening[rush]",
            "value": 1635.8543547519555,
            "unit": "iter/sec",
            "range": "stddev: 0.00002163649488536787",
            "extra": "mean: 611.3013649993491 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetListening::test_listening[original]",
            "value": 764.2383814165796,
            "unit": "iter/sec",
            "range": "stddev: 0.000023515800487176186",
            "extra": "mean: 1.3084922509995067 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3Api::test_api[rush]",
            "value": 566753.6635234163,
            "unit": "iter/sec",
            "range": "stddev: 4.754798684871882e-7",
            "extra": "mean: 1.7644349994725417 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3Api::test_api[original]",
            "value": 1024361.3621419993,
            "unit": "iter/sec",
            "range": "stddev: 5.6348288951263424e-8",
            "extra": "mean: 976.2179997778729 nsec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToHex::test_to_hex[rush]",
            "value": 101268.00709550685,
            "unit": "iter/sec",
            "range": "stddev: 3.3987184342020505e-7",
            "extra": "mean: 9.874787000171636 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToHex::test_to_hex[original]",
            "value": 123549.04008255218,
            "unit": "iter/sec",
            "range": "stddev: 2.1412114731588717e-7",
            "extra": "mean: 8.093951999398996 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToInt::test_to_int[rush]",
            "value": 124100.99686709311,
            "unit": "iter/sec",
            "range": "stddev: 3.151766472719512e-7",
            "extra": "mean: 8.05795299993406 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToInt::test_to_int[original]",
            "value": 155542.30680983505,
            "unit": "iter/sec",
            "range": "stddev: 5.026906185364362e-7",
            "extra": "mean: 6.429118999903949 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToWei::test_to_wei[rush]",
            "value": 65586.73731194134,
            "unit": "iter/sec",
            "range": "stddev: 0.0000025569876395497515",
            "extra": "mean: 15.246985000089808 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToWei::test_to_wei[original]",
            "value": 68849.31858800298,
            "unit": "iter/sec",
            "range": "stddev: 3.277490074543537e-7",
            "extra": "mean: 14.524471999266098 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3FromWei::test_from_wei[rush]",
            "value": 139591.98656347138,
            "unit": "iter/sec",
            "range": "stddev: 2.6593199985918194e-7",
            "extra": "mean: 7.163735000972338 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3FromWei::test_from_wei[original]",
            "value": 149960.4929066894,
            "unit": "iter/sec",
            "range": "stddev: 2.4838482266921815e-7",
            "extra": "mean: 6.668423000064649 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3IsAddress::test_is_address[rush]",
            "value": 138807.30104318637,
            "unit": "iter/sec",
            "range": "stddev: 5.042631080607265e-7",
            "extra": "mean: 7.204231999935473 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3IsAddress::test_is_address[original]",
            "value": 20195.766450532377,
            "unit": "iter/sec",
            "range": "stddev: 0.000001920710142726871",
            "extra": "mean: 49.51532799952929 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3IsChecksumAddress::test_is_checksum_address[rush]",
            "value": 30354.158663523744,
            "unit": "iter/sec",
            "range": "stddev: 9.029338410125361e-7",
            "extra": "mean: 32.94441500042922 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3IsChecksumAddress::test_is_checksum_address[original]",
            "value": 11480.854056984697,
            "unit": "iter/sec",
            "range": "stddev: 0.0000017675731405817401",
            "extra": "mean: 87.10153400056697 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToChecksumAddress::test_to_checksum_address[rush]",
            "value": 29065.84270787359,
            "unit": "iter/sec",
            "range": "stddev: 8.446527957165908e-7",
            "extra": "mean: 34.40464500033613 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToChecksumAddress::test_to_checksum_address[original]",
            "value": 12064.566568305245,
            "unit": "iter/sec",
            "range": "stddev: 0.0000011428382877913466",
            "extra": "mean: 82.88735399969482 usec\nrounds: 100"
          }
        ]
      }
    ]
  }
}