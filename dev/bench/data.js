window.BENCHMARK_DATA = {
  "lastUpdate": 1679779426240,
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
      }
    ]
  }
}