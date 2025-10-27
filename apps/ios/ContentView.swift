import SwiftUI

struct ContentView: View {
    @State private var transformer: TransformerBridge?
    @State private var status = "Not initialized"
    @State private var backend = ""
    @State private var inputText = "1, 2, 3"
    @State private var results: [(Int, Float)] = []
    @State private var isProcessing = false
    
    var body: some View {
        NavigationView {
            VStack(spacing: 20) {
                // Status section
                VStack(alignment: .leading, spacing: 8) {
                    HStack {
                        Circle()
                            .fill(statusColor)
                            .frame(width: 12, height: 12)
                        Text("Status")
                            .font(.headline)
                    }
                    Text(status)
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                    
                    if !backend.isEmpty {
                        Text("Backend: \(backend)")
                            .font(.caption)
                            .foregroundColor(.blue)
                    }
                }
                .frame(maxWidth: .infinity, alignment: .leading)
                .padding()
                .background(Color.gray.opacity(0.1))
                .cornerRadius(10)
                
                Divider()
                
                // Input section
                VStack(alignment: .leading, spacing: 8) {
                    Text("Input Token IDs")
                        .font(.headline)
                    
                    TextField("e.g., 1, 2, 3", text: $inputText)
                        .textFieldStyle(RoundedBorderTextFieldStyle())
                        .keyboardType(.numbersAndPunctuation)
                    
                    Text("Enter comma-separated token IDs")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
                
                // Buttons
                VStack(spacing: 12) {
                    Button(action: initializeEngine) {
                        HStack {
                            Image(systemName: "power")
                            Text("Initialize Engine")
                        }
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(Color.blue)
                        .foregroundColor(.white)
                        .cornerRadius(10)
                    }
                    .disabled(transformer != nil)
                    
                    Button(action: createModel) {
                        HStack {
                            Image(systemName: "cube.box")
                            Text("Create Model")
                        }
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(Color.green)
                        .foregroundColor(.white)
                        .cornerRadius(10)
                    }
                    .disabled(transformer == nil)
                    
                    Button(action: runInference) {
                        HStack {
                            if isProcessing {
                                ProgressView()
                                    .progressViewStyle(CircularProgressViewStyle(tint: .white))
                            } else {
                                Image(systemName: "play.circle")
                                Text("Run Inference")
                            }
                        }
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(Color.purple)
                        .foregroundColor(.white)
                        .cornerRadius(10)
                    }
                    .disabled(transformer == nil || isProcessing)
                }
                
                // Results section
                if !results.isEmpty {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Top 5 Predictions")
                            .font(.headline)
                        
                        ForEach(results.indices, id: \.self) { index in
                            HStack {
                                Text("\(index + 1).")
                                    .font(.caption)
                                    .foregroundColor(.secondary)
                                Text("Token \(results[index].0)")
                                    .font(.body)
                                Spacer()
                                Text(String(format: "%.2f%%", results[index].1 * 100))
                                    .font(.body)
                                    .foregroundColor(.blue)
                            }
                            .padding(.vertical, 4)
                        }
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .padding()
                    .background(Color.green.opacity(0.1))
                    .cornerRadius(10)
                }
                
                Spacer()
            }
            .padding()
            .navigationTitle("🚀 Tiny Transformer")
        }
    }
    
    private var statusColor: Color {
        if transformer != nil && !backend.isEmpty {
            return .green
        } else if transformer != nil {
            return .yellow
        } else {
            return .red
        }
    }
    
    private func initializeEngine() {
        status = "Initializing..."
        
        DispatchQueue.global(qos: .userInitiated).async {
            if let bridge = TransformerBridge() {
                let backendName = bridge.getBackend()
                
                DispatchQueue.main.async {
                    self.transformer = bridge
                    self.backend = backendName
                    self.status = "Engine initialized"
                }
            } else {
                DispatchQueue.main.async {
                    self.status = "Failed to initialize"
                }
            }
        }
    }
    
    private func createModel() {
        guard let transformer = transformer else { return }
        
        status = "Creating model..."
        
        DispatchQueue.global(qos: .userInitiated).async {
            let success = transformer.createDefaultModel()
            
            DispatchQueue.main.async {
                if success {
                    let vocabSize = transformer.getVocabSize()
                    self.status = "Model created (vocab: \(vocabSize))"
                } else {
                    self.status = "Failed to create model"
                }
            }
        }
    }
    
    private func runInference() {
        guard let transformer = transformer else { return }
        
        // Parse input
        let tokens = inputText.split(separator: ",")
            .compactMap { Float($0.trimmingCharacters(in: .whitespaces)) }
        
        guard !tokens.isEmpty else {
            status = "Invalid input"
            return
        }
        
        isProcessing = true
        status = "Running inference..."
        
        DispatchQueue.global(qos: .userInitiated).async {
            if let output = transformer.forward(input: tokens) {
                // Get top 5
                let indexed = output.enumerated().map { ($0.offset, $0.element) }
                let sorted = indexed.sorted { $0.1 > $1.1 }
                let top5 = Array(sorted.prefix(5))
                
                DispatchQueue.main.async {
                    self.results = top5
                    self.status = "Inference complete"
                    self.isProcessing = false
                }
            } else {
                DispatchQueue.main.async {
                    self.status = "Inference failed"
                    self.isProcessing = false
                }
            }
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
