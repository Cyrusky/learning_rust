# 需要在当前的设计中做出的改动

1. 在 generalInfo 中添加一个可以记录当前 `register` 状态的值, 或者单独加一张表, 表里面的字段可以将当前 prodName 和状态进行绑定(比较推荐).
2.

# 设计(下面是需要增加的)

## PS:

1. 流程中的状态因为存在一个状态对应多种处理的情况, 还是不要用有限状态机来做了吧, 直接用多个接口来做,比较清晰, 扩展也没啥问题.
2. 流程控制和其他的操作分开, 如果需要验证的话, 只需要单独判断一下就行了. 下面会有说明.

## 用户的编辑

```graphql
mutation updateRegister{

}
variables registerInfo{

}
```

- 用户始终编辑的是一份文档, 状态不变, 流程也不会变.

## 用户的提交

```graphql
mutation Workflow {

}
variables WorkflowInfo {
    // 任何能够定为到当前register的值,可以是ID,也可以是名字.
    // 按照需求,应该是ProdName, 但是registerId 也是一个选择.
    registerId,
    actionType,
}
```

- 用户在做提交动作, 通过 `registerId` 或者任何给定的唯一值定为到当前进入流程的 `register` 记录.
- 通过 `ActionType` 来判断当前状态的下一个状态
  - 如果 `ActionType` 是`submit`, 那么会判断当前状态是否为`DraftState`, 如果是,那么进入`SubmittedState`状态.
  - 否则, 返回错误信息.

## 分配 review leader

```
接口同用户提交
```

- 通过`actionType`来判断, 如果是`assigned_review_leader`, 那么判断当前的状态是否为`SubmittedState`

  - 如果是, 那么判断是否已经分配了`ReviewLeader`, 如果已经分配,那么将状态改变为`ReadyForReview`, 否则不与分配.
  -

- 用户对 `register` 进行编辑的时候,自动迭代版本, 建议前端对用户版本做正则验证, 不然的话,如果出现类似于"X.Y.Z"这种非数字的版本号,不太好自动加 1.
