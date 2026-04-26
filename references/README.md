# Argent Engine - References and Research Sources

## 1. Knowledge Graph with LLM Integration

### 1.1 GraphRAG & Knowledge Graph Retrieval Augmented Generation
- **GraphRAG** (Microsoft Research 2024): https://arxiv.org/abs/2505.20099
  - Graph-based retrieval for enhanced LLM performance
  - Community detection paradigm
- **KG-RAG** (2024): Knowledge graph retrieval augmented generation
- **LightRAG** (Oct 2024): 10x token reduction with dual-level retrieval

### 1.2 Dynamic Knowledge Graph Construction
- **TOBUGraph** (arXiv:2412.05447v2): https://arxiv.org/abs/2412.05447v2
  - Knowledge graph-based retrieval beyond RAG
  - Dynamic automatic KG construction from unstructured data
  - Graph traversal retrieval
  
- **Agentic-KGR** (2025): https://arxiv.org/abs/2510.09156
  - Co-evolution between LLMs and Knowledge Graphs
  - Multi-round reinforcement learning
  - Dynamic schema expansion
  - Up to +33.3 points improvement over RL methods

### 1.3 LLM Knowledge Graph Builders
- **Neo4j LLM Knowledge Graph Builder** (2025): https://neo4j.com/blog/developer/llm-knowledge-graph-builder-release/
  - Community summaries
  - Parallel retrievers
  - Multiple model support (GPT-4o, Gemini, Claude, etc.)

### 1.4 Knowledge Graph Papers Repository
- **KG-LLM-Papers** (GitHub): https://github.com/zjukg/kg-llm-papers
  - Comprehensive collection of KG+LLM integration papers

## 2. Self-Learning & Feedback Systems

### 2.1 Reinforcement Learning from Human Feedback (RLHF)
- **RLHF Book** (arXiv:2504.12501): https://arxiv.org/abs/2504.12501
  - Comprehensive guide to RLHF methods
  - Origins from economics, philosophy, optimal control
  
- **MA-RLHF** (arXiv:2410.02743v1): https://arxiv.org/abs/2410.02743v1
  - Macro actions for faster credit assignment
  - 30% performance improvement in text summarization

### 2.2 Nash Learning from Human Feedback (NLHF)
- **NLHF** (NeurIPS 2024): https://proceedings.mlr.press/v235/munos24a.html
  - Mirror descent-based policy optimization
  - Nash equilibrium for preference alignment

### 2.3 Multi-turn Preference Optimization
- **Multi-turn RLHF** (NeurIPS 2024): https://arxiv.org/pdf/2405.14655
  - Conversation-level feedback
  - MTPO algorithm

### 2.4 Self-Evolved Reward Learning (SER)
- **SER** (arXiv:2411.00418): https://arxiv.org/abs/2411.00418
  - Self-feedback loop for RM improvement
  - Reduces need for human-annotated data

### 2.5 RLAIF - Reinforcement Learning from AI Feedback
- **RLAIF vs RLHF** (NeurIPS 2024): https://proceedings.mlr.press/v235/lee24t.html
  - AI-generated feedback for scaling
  - Comparable performance to RLHF

### 2.6 Online Iterative RLHF
- **Online Iterative RLHF** (arXiv:2405.07863): https://arxiv.org/pdf/2405.07863
  - On-policy sampling
  - Works with limited human feedback via proxy models

## 3. Episodic Memory & Knowledge Graphs for Agents

### 3.1 AriGraph
- **AriGraph** (IJCAI 2025): https://www.ijcai.org/proceedings/2025/2
  - Knowledge graph world models with episodic memory
  - Semantic + episodic memory integration
  - Outperforms other memory methods in decision-making

## 4. Implementation Patterns

### 4.1 Production-Ready Systems
- **FalkorDB**: Performance-critical deployments
- **Cognee**: Agentic systems with unified memory
- **AutoSchemaKG**: Dynamic schema discovery

### 4.2 Key Architectural Patterns
- Community detection for global context
- Local + global retrievers
- Vector embeddings + graph structures
- Dynamic schema expansion

## 5. Citations & BibTeX References

### Knowledge Graph + LLM
```bibtex
@article{graphrag2024,
  title={Large Language Models Meet Knowledge Graphs},
  author={Zhang et al.},
  journal={arXiv:2505.20099},
  year={2025}
}

@article{tobugraph2024,
  title={TOBUGraph: Knowledge Graph-Based Retrieval for Enhanced LLM Performance},
  author={Anonymous},
  journal={arXiv:2412.05447v2},
  year={2024}
}

@article{agentickgr2025,
  title={Agentic-KGR: Co-evolutionary Knowledge Graph},
  author={Anonymous},
  journal={arXiv:2510.09156},
  year={2025}
}

@article{arigraph2025,
  title={AriGraph: Learning Knowledge Graph World Models with Episodic Memory},
  author={Anokhin et al.},
  journal={IJCAI 2025},
  year={2025}
}
```

### RLHF & Self-Learning
```bibtex
@article{rlhf2024,
  title={Reinforcement Learning from Human Feedback},
  author={Chai},
  journal={arXiv:2504.12501},
  year={2025}
}

@article{marlhf2024,
  title={MA-RLHF: RLHF with Macro Actions},
  author={Chai},
  journal={arXiv:2410.02743v1},
  year={2024}
}

@article{nlhf2024,
  title={Nash Learning from Human Feedback},
  author={Munos et al.},
  journal={NeurIPS 2024},
  year={2024}
}

@article{ser2024,
  title={Self-Evolved Reward Learning},
  author={Huang et al.},
  journal={arXiv:2411.00418},
  year={2024}
}

@article{rlaif2024,
  title={RLAIF vs RLHF: Scaling RLHF with AI Feedback},
  author={Lee et al.},
  journal={NeurIPS 2024},
  year={2024}
}
```

## 6. Key Performance Metrics from Papers

| Paper | Improvement | Key Metric |
|-------|-------------|------------|
| Agentic-KGR | +33.3% | KG extraction |
| Agentic-KGR | +12.8% | QA tasks |
| MA-RLHF | 30% | Text summarization |
| MA-RLHF | 18% | Dialogue generation |
| MA-RLHF | 8% | Question answering |
| LightRAG | 10x | Token reduction |

## 7. Implementation Notes for Argent Engine

Based on the research, Argent Engine should implement:

1. **Knowledge Graph Integration** 
   - Dynamic schema expansion
   - Entity + relation extraction
   - Graph traversal for retrieval

2. **Memory System**
   - Semantic memory (facts, concepts)
   - Episodic memory (experiences, interactions)
   - Working memory (current context)

3. **Self-Learning Loop**
   - Feedback collection
   - Reward modeling
   - Policy optimization
   - Iterative improvement

4. **Performance Tracking**
   - Success/failure logging
   - Weight adjustment based on outcomes
   - Continuous learning from interactions

---
*References last updated: April 2026*