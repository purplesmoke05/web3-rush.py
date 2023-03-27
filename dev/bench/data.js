window.BENCHMARK_DATA = {
  "lastUpdate": 1679934519450,
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
          "id": "ad89e414bfe82c0baa26b09e1ba659aabca4ddfc",
          "message": "add address method",
          "timestamp": "2023-03-28T01:21:46+09:00",
          "tree_id": "f051f2e0e8153c1e458094ea5f616e0848b695ff",
          "url": "https://github.com/purplesmoke05/web3-rush.py/commit/ad89e414bfe82c0baa26b09e1ba659aabca4ddfc"
        },
        "date": 1679934517417,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthBlockNumber::test_block_number[rush]",
            "value": 1169.6717801910138,
            "unit": "iter/sec",
            "range": "stddev: 0.00006796928981825631",
            "extra": "mean: 854.9406910002517 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthBlockNumber::test_block_number[original]",
            "value": 554.1778107136612,
            "unit": "iter/sec",
            "range": "stddev: 0.00009697769894705273",
            "extra": "mean: 1.8044749910001203 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthChainId::test_chain_id[rush]",
            "value": 1231.1323731500938,
            "unit": "iter/sec",
            "range": "stddev: 0.00006787220527422228",
            "extra": "mean: 812.2603400000799 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthChainId::test_chain_id[original]",
            "value": 562.4564866788196,
            "unit": "iter/sec",
            "range": "stddev: 0.00012998086977781528",
            "extra": "mean: 1.7779153120000046 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSyncing::test_syncing[rush]",
            "value": 1305.6014191573124,
            "unit": "iter/sec",
            "range": "stddev: 0.000057527356955594985",
            "extra": "mean: 765.9305400000562 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSyncing::test_syncing[original]",
            "value": 544.0305896997403,
            "unit": "iter/sec",
            "range": "stddev: 0.0001373738471418225",
            "extra": "mean: 1.8381319340001028 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthAccounts::test_accounts[rush]",
            "value": 950.6334888483483,
            "unit": "iter/sec",
            "range": "stddev: 0.00010625760599996474",
            "extra": "mean: 1.0519301199997244 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthAccounts::test_accounts[original]",
            "value": 326.870297886354,
            "unit": "iter/sec",
            "range": "stddev: 0.00013847463763363264",
            "extra": "mean: 3.059317431000352 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSendTransaction::test_send_transaction[rush]",
            "value": 131.650873108017,
            "unit": "iter/sec",
            "range": "stddev: 0.0026604309735212654",
            "extra": "mean: 7.595847839000044 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthSendTransaction::test_send_transaction[original]",
            "value": 46.22146996348503,
            "unit": "iter/sec",
            "range": "stddev: 0.001139991968348567",
            "extra": "mean: 21.63496748999978 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthWaitForTransactionReceipt::test_wait_for_transaction_receipt[rush]",
            "value": 936.4340670509189,
            "unit": "iter/sec",
            "range": "stddev: 0.0010348195902488554",
            "extra": "mean: 1.0678808420001928 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthWaitForTransactionReceipt::test_wait_for_transaction_receipt[original]",
            "value": 312.47643869065234,
            "unit": "iter/sec",
            "range": "stddev: 0.0010834442152017135",
            "extra": "mean: 3.200241285999766 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransaction::test_get_transaction[rush]",
            "value": 761.9518008903849,
            "unit": "iter/sec",
            "range": "stddev: 0.00026809859105015135",
            "extra": "mean: 1.3124189729999216 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransaction::test_get_transaction[original]",
            "value": 280.761877305734,
            "unit": "iter/sec",
            "range": "stddev: 0.00017461531664440016",
            "extra": "mean: 3.5617371190001563 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransactionCount::test_get_transaction_count[rush]",
            "value": 1183.693628801799,
            "unit": "iter/sec",
            "range": "stddev: 0.00009527921962766734",
            "extra": "mean: 844.8131979997697 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetTransactionCount::test_get_transaction_count[original]",
            "value": 335.5412228771568,
            "unit": "iter/sec",
            "range": "stddev: 0.00021967361814654793",
            "extra": "mean: 2.9802597470001615 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBlock::test_get_block[rush]",
            "value": 899.6405595897838,
            "unit": "iter/sec",
            "range": "stddev: 0.0001884692571352351",
            "extra": "mean: 1.1115550420003046 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBlock::test_get_block[original]",
            "value": 393.5081431661649,
            "unit": "iter/sec",
            "range": "stddev: 0.0003247585306651378",
            "extra": "mean: 2.5412434720003603 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBalance::test_get_balance[rush]",
            "value": 1184.5450142399786,
            "unit": "iter/sec",
            "range": "stddev: 0.00021686773884186923",
            "extra": "mean: 844.205993000287 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetBalance::test_get_balance[original]",
            "value": 347.3689011529733,
            "unit": "iter/sec",
            "range": "stddev: 0.00014832252964237308",
            "extra": "mean: 2.8787839000003714 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetCode::test_get_code[rush]",
            "value": 1136.366655483603,
            "unit": "iter/sec",
            "range": "stddev: 0.00008989610809446019",
            "extra": "mean: 879.9976619997096 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetCode::test_get_code[original]",
            "value": 337.91269440412185,
            "unit": "iter/sec",
            "range": "stddev: 0.00016128912131785342",
            "extra": "mean: 2.9593442819998472 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthEstimateGas::test_estimate_gas[rush]",
            "value": 1063.8584480342167,
            "unit": "iter/sec",
            "range": "stddev: 0.000127361043216366",
            "extra": "mean: 939.974675999224 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthEstimateGas::test_estimate_gas[original]",
            "value": 178.40171056407615,
            "unit": "iter/sec",
            "range": "stddev: 0.00029282062307733507",
            "extra": "mean: 5.605327420001572 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetLogs::test_get_logs[rush]",
            "value": 585.3056225692577,
            "unit": "iter/sec",
            "range": "stddev: 0.00024037548187710837",
            "extra": "mean: 1.7085091300001523 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_eth.py::TestEthGetLogs::test_get_logs[original]",
            "value": 24.036107967837015,
            "unit": "iter/sec",
            "range": "stddev: 0.0012437522897610087",
            "extra": "mean: 41.60407339400001 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetVersion::test_version[rush]",
            "value": 1159.2962377661943,
            "unit": "iter/sec",
            "range": "stddev: 0.0001691975045383374",
            "extra": "mean: 862.5922929991248 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetVersion::test_version[original]",
            "value": 568.9874018604751,
            "unit": "iter/sec",
            "range": "stddev: 0.0001504688444215986",
            "extra": "mean: 1.757508156999961 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetListening::test_listening[rush]",
            "value": 1234.96162377088,
            "unit": "iter/sec",
            "range": "stddev: 0.00029101744310752964",
            "extra": "mean: 809.7417610002822 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_net.py::TestNetListening::test_listening[original]",
            "value": 573.3865252087002,
            "unit": "iter/sec",
            "range": "stddev: 0.00010374573734206031",
            "extra": "mean: 1.7440242419997958 msec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3Api::test_api[rush]",
            "value": 583357.0563329431,
            "unit": "iter/sec",
            "range": "stddev: 4.30130173905183e-7",
            "extra": "mean: 1.7142160005505502 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3Api::test_api[original]",
            "value": 911070.4164537475,
            "unit": "iter/sec",
            "range": "stddev: 0.0000012540281393703947",
            "extra": "mean: 1.0976100002153544 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToHex::test_to_hex[rush]",
            "value": 43529.381396811106,
            "unit": "iter/sec",
            "range": "stddev: 0.000006165592451077531",
            "extra": "mean: 22.972988999867994 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToHex::test_to_hex[original]",
            "value": 93354.2509083557,
            "unit": "iter/sec",
            "range": "stddev: 0.0000033856284529153518",
            "extra": "mean: 10.71188500009157 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToInt::test_to_int[rush]",
            "value": 41953.00114453235,
            "unit": "iter/sec",
            "range": "stddev: 0.0000034721894522038857",
            "extra": "mean: 23.836196999468484 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToInt::test_to_int[original]",
            "value": 128130.13922702357,
            "unit": "iter/sec",
            "range": "stddev: 0.0000032204803568072286",
            "extra": "mean: 7.804564999560171 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToWei::test_to_int[rush]",
            "value": 23165.11820607598,
            "unit": "iter/sec",
            "range": "stddev: 0.000006544864380855908",
            "extra": "mean: 43.16835299971444 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToWei::test_to_int[original]",
            "value": 52961.73403500243,
            "unit": "iter/sec",
            "range": "stddev: 0.000016033651806779415",
            "extra": "mean: 18.88155699998606 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3FromWei::test_to_int[rush]",
            "value": 34039.14406152625,
            "unit": "iter/sec",
            "range": "stddev: 0.00000835083868350851",
            "extra": "mean: 29.377942000905932 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3FromWei::test_to_int[original]",
            "value": 127912.14379084727,
            "unit": "iter/sec",
            "range": "stddev: 0.0000015189716267086667",
            "extra": "mean: 7.817866000550567 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3IsAddress::test_is_address[rush]",
            "value": 122401.01338088153,
            "unit": "iter/sec",
            "range": "stddev: 0.0000015957171550524056",
            "extra": "mean: 8.169867000106024 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3IsAddress::test_is_address[original]",
            "value": 15697.496680912167,
            "unit": "iter/sec",
            "range": "stddev: 0.000013396152654259583",
            "extra": "mean: 63.70442500019636 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3IsChecksumAddress::test_is_checksum_address[rush]",
            "value": 27334.1289202737,
            "unit": "iter/sec",
            "range": "stddev: 0.000006321506666426884",
            "extra": "mean: 36.58430100028909 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3IsChecksumAddress::test_is_checksum_address[original]",
            "value": 9328.211585464782,
            "unit": "iter/sec",
            "range": "stddev: 0.00001221151409830848",
            "extra": "mean: 107.20168500017735 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToChecksumAddress::test_to_checksum_address[rush]",
            "value": 26185.05987324177,
            "unit": "iter/sec",
            "range": "stddev: 0.000005950283404275044",
            "extra": "mean: 38.18971599991983 usec\nrounds: 100"
          },
          {
            "name": "tests/benchmark/test_bench_web3_root.py::TestWeb3ToChecksumAddress::test_to_checksum_address[original]",
            "value": 9376.168270601755,
            "unit": "iter/sec",
            "range": "stddev: 0.00002512371933511985",
            "extra": "mean: 106.65337599959912 usec\nrounds: 100"
          }
        ]
      }
    ]
  }
}