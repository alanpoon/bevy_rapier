use crate::physics;
use crate::physics::{
    JointsEntityMap, ModificationTracker, PhysicsHooksWithQueryObject, RapierConfiguration,
    SimulationToRenderTime,
};
use crate::prelude::IntersectionEvent;
use crate::rapier::geometry::ContactEvent;
use crate::rapier::pipeline::QueryPipeline;
use bevy_ecs::event::Events;
use bevy_ecs::query::WorldQuery;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use crate::physics::time::{TimeInterface,Time};

use rapier::dynamics::{
    CCDSolver, ImpulseJointSet, IntegrationParameters, IslandManager, MultibodyJointSet,
};
use rapier::geometry::{BroadPhase, NarrowPhase};
use rapier::pipeline::PhysicsPipeline;
use std::marker::PhantomData;

#[derive(Component)]
pub struct S;
pub type NoUserData<'a> = &'a S;
//pub type NoUserData = S;
/// A plugin responsible for setting up a full Rapier physics simulation pipeline and resources.
///
/// This will automatically setup all the resources needed to run a Rapier physics simulation including:
/// - The physics pipeline.
/// - The integration parameters.
/// - The rigid-body, collider, and joint, sets.
/// - The gravity.
/// - The broad phase and narrow-phase.
/// - The event queue.
/// - Systems responsible for executing one physics timestep at each Bevy update stage.
pub struct RapierPhysicsPlugin<UserData,X>(PhantomData<UserData>,PhantomData<X>);

impl<UserData,X> Default for RapierPhysicsPlugin<UserData,X> {
    fn default() -> Self {
        Self(PhantomData,PhantomData)
    }
}

/// The stage where the physics transform are output to the Bevy Transform.
///
/// This stage is added right before the `POST_UPDATE` stage.
pub const TRANSFORM_SYNC_STAGE: &'static str = "rapier::transform_sync_stage";

/// The names of the default App stages
#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum PhysicsStages {
    FinalizeCreations,
    SyncTransforms,
}

impl<UserData: 'static + WorldQuery + Send + Sync,X:'static + Component+ TimeInterface + Send + Sync> Plugin for RapierPhysicsPlugin<UserData,X> {
    fn build(&self, app: &mut App) {
        app.add_stage_before(
            CoreStage::PreUpdate,
            PhysicsStages::FinalizeCreations,
            SystemStage::single_threaded(),
        )
        .add_stage_before(
            CoreStage::PostUpdate,
            PhysicsStages::SyncTransforms,
            SystemStage::single_threaded(),
        )
        .insert_resource(PhysicsPipeline::new())
        .insert_resource(QueryPipeline::new())
        .insert_resource(RapierConfiguration::default())
        .insert_resource(IntegrationParameters::default())
        .insert_resource(BroadPhase::new())
        .insert_resource(NarrowPhase::new())
        .insert_resource(IslandManager::new())
        .insert_resource(ImpulseJointSet::new())
        .insert_resource(MultibodyJointSet::new())
        .insert_resource(CCDSolver::new())
        .insert_resource(Events::<IntersectionEvent>::default())
        .insert_resource(Events::<ContactEvent>::default())
        .insert_resource(SimulationToRenderTime::default())
        .insert_resource(JointsEntityMap::default())
        .insert_resource(ModificationTracker::default())
        .insert_resource(Time::default())
        .add_system_to_stage(
            PhysicsStages::FinalizeCreations,
            physics::attach_bodies_and_colliders_system
                .label(physics::PhysicsSystems::AttachBodiesAndColliders),
        )
        .add_system_to_stage(
            PhysicsStages::FinalizeCreations,
            physics::create_joints_system.label(physics::PhysicsSystems::CreateJoints),
        )
        .add_system_to_stage(
            CoreStage::PreUpdate,
            physics::finalize_collider_attach_to_bodies
                .label(physics::PhysicsSystems::FinalizeColliderAttachToBodies),
        )
        .add_system_to_stage(
            CoreStage::Update,
            physics::step_world_system::<UserData,X>.label(physics::PhysicsSystems::StepWorld),
        )
        .add_system_to_stage(
            PhysicsStages::SyncTransforms,
            physics::sync_transforms.label(physics::PhysicsSystems::SyncTransforms),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            physics::collect_removals.label(physics::PhysicsSystems::CollectRemovals),
        )
        ;
        if app
            .world
            .get_resource::<PhysicsHooksWithQueryObject<UserData>>()
            .is_none()
        {
            app.insert_resource(PhysicsHooksWithQueryObject::<UserData>(Box::new(())));
        }
    }
}
