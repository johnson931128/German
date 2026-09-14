# German — Distributed World Simulator

German 是一個以 C++ 開發的小型分散式系統模擬平台，用於視覺化多個獨立 Server 共同維護虛擬世界的過程。
系統以 2D 模擬世界作為主要呈現方式，透過 SFML 顯示 Server、節點、物件、連線以及即時事件。
每個 Server 都是獨立執行的 Process，透過網路協定彼此交換狀態與訊息，而不是單一程式內的模擬物件。
Server 可由使用者手動建立與啟動，並設定不同角色，例如管理特定區域、執行路徑計算或提供其他客製化服務。
虛擬世界中的 Entity 可以在不同 Server 管理的區域之間移動，使狀態同步與 Server 間通訊能直接被觀察。
Visualization Client 負責收集系統狀態，並以圖形方式呈現節點連線、資料流、Entity 狀態與 Server 行為。
系統將支援 Server 上線、離線、延遲與故障等事件，用來觀察分散式系統在不同網路狀況下的反應。
後續可逐步加入 Heartbeat、Service Discovery、Replication、Leader Election 與 Consensus 等分散式機制。
專案重點不是建立大型遊戲，而是建立一個可以互動、觀察並實驗分散式系統概念的視覺化 Sandbox。
