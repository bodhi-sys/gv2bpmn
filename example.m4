include(bpmn.m4)

digraph developer_workflow {
graph [rankdir="TB", splines="ortho", overlap="false", sep="0.8", fontsize=10, ranksep=1.0, nodesep=0.8]
node [margin="0.4,0.4", fontsize=10, shape="box", style="rounded"]
edge [arrowsize=0.9, penwidth=0.9, minlen=2, splines="ortho"]

  // BPMN: Производство сметаны — от коровы до продукта

  start([[Ферма: Начало]])
  t(milking, [[Доение коровы]])
  t(transport_milk, [[Транспортировка молока на завод]])
  t(pasteurization, [[Пастеризация молока]])
  t(standardization, [[Стандартизация жирности]])
  t(culture_addition, [[Добавление закваски (мезофильные культуры)]])
  t(fermentation, [[Ферментация/созревание]])
  t(chilling, [[Охлаждение до температур хранения]])
  t(packaging, [[Разлив, упаковка сметаны]])
  t(labelling, [[Маркировка и нанесение информации]])
  t(storage, [[Хранение готовой продукции на складе]])
  t(distribution, [[Дистрибуция: отгрузка дистрибьюторам/магазинам]])
  t(retail, [[Продажа в рознице]])
  t(consumer, [[Потребительский продукт (сметана)]])
  end([[Потребитель: Конец]])

  g_xor(quality_decision, [[Соответствует качеству?]])
  g_xor(shelflife_decision, [[Соответствует срокам годности?]])
  g_xor(receiving, [[Прием и контроль качества молока]])


  
  to(START_NODE, milking)
  to(milking, transport_milk)
  to(transport_milk, receiving)

  g_to(receiving, transport_milk, [[Не принято: возврат/переработка]])
  g_to(receiving, pasteurization, [[Принято]])

  to(pasteurization, standardization)
  to(standardization, culture_addition)
  to(culture_addition, fermentation)

  to(fermentation, quality_decision)
  g_to(quality_decision, pasteurization, [[Проблемы с микробиологией]])
  g_to(quality_decision, chilling, [[OK]])

  to(chilling, packaging)
  to(packaging, labelling)
  to(labelling, storage)

  to(storage, shelflife_decision)
  g_to(shelflife_decision, packaging, [[Низкий срок: переработать/перепаковать]])
  g_to(shelflife_decision, distribution, [[OK]])

  to(distribution, retail)
  to(retail, consumer)

  to(consumer, END_NODE)

}


