export type ClientData = {
      id: string,
      mac: string,
      credits: number,
      active: boolean
      remainingSeconds: number,
}

export type CodeData = {
      code: string,
      kind: string,
      units: number
}

export type DataPlanData=  {
      id: number,
      name: string,
      creditCost: number,
      megabytesGiven: number,
}

export type PlanData = {
      id: number,
      name: string,
      creditCost: number,
      secondsGiven: number,
}

export type PlanCollectionData = {
      timePlans: PlanData[],
      dataPlans: DataPlanData[],
}