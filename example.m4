include(bpmn.m4)

digraph developer_workflow {
graph [rankdir="TB", splines="ortho", overlap="false", sep="0.8", fontsize=10, ranksep=1.0, nodesep=0.8]
node [margin="0.4,0.4", fontsize=10, shape="box", style="rounded"]
edge [arrowsize=0.9, penwidth=0.9, minlen=2, splines="ortho"]

  // BPMN: Производство сметаны — от коровы до продукта

  start([[Ферма: Начало]])

  subgraph cluster_milk {
    graph[label="молоко"]
    color="lightblue"
    t(milk:milking, [[Доение коровы]])
    t(milk:transport_milk, [[Транспортировка молока на завод]])
    t(milk:pasteurization, [[Пастеризация молока]])
    t(milk:standardization, [[Стандартизация жирности]])
    g_xor(milk:receiving, [[Прием и контроль качества молока]])
  }

  subgraph cluster_smetana {
    graph[label="сметана"]
    color="orange"
    t(smetana:culture_addition, [[Добавление закваски (мезофильные культуры)]])
    t(smetana:fermentation, [[Ферментация/созревание]])
    t(smetana:chilling, [[Охлаждение до температур хранения]])
    t(smetana:packaging, [[Разлив, упаковка сметаны]])
    g_xor(smetana:quality_decision, [[Соответствует качеству?]])
    g_xor(smetana:shelflife_decision, [[Соответствует срокам годности?]])
  }

  subgraph cluster_sklad {
    graph[label="склад"]
    color="lightgreen"
    t(sklad:labelling, [[Маркировка и нанесение информации]])
    t(sklad:storage, [[Хранение готовой продукции на складе]])
    t(sklad:distribution, [[Дистрибуция: отгрузка дистрибьюторам/магазинам]])
    t(sklad:retail, [[Продажа в рознице]])
    t(sklad:consumer, [[Потребительский продукт (сметана)]])
  }

  end([[Потребитель: Конец]])



  
  to(START_NODE, milk:milking)
  to(milk:milking, milk:transport_milk)
  to(milk:transport_milk, milk:receiving)

  g_to(milk:receiving, milk:transport_milk, [[Не принято: возврат/переработка]])
  g_to(milk:receiving, milk:pasteurization, [[Принято]])

  to(milk:pasteurization, milk:standardization)
  to(milk:standardization, smetana:culture_addition)
  to(smetana:culture_addition, smetana:fermentation)

  to(smetana:fermentation, smetana:quality_decision)
  g_to(smetana:quality_decision, milk:pasteurization, [[Проблемы с микробиологией]])
  g_to(smetana:quality_decision, smetana:chilling, [[OK]])

  to(smetana:chilling, smetana:packaging)
  to(smetana:packaging, sklad:labelling)
  to(sklad:labelling, sklad:storage)

  to(sklad:storage, smetana:shelflife_decision)
  g_to(smetana:shelflife_decision, smetana:packaging, [[Низкий срок: переработать/перепаковать]])
  g_to(smetana:shelflife_decision, sklad:distribution, [[OK]])

  to(sklad:distribution, sklad:retail)
  to(sklad:retail, sklad:consumer)

  to(sklad:consumer, END_NODE)

}


